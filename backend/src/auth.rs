use std::ops::Deref;
use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use chrono::Duration;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use openidconnect::{AuthorizationCode, ClientId, ClientSecret, CsrfToken, EmptyAdditionalClaims, IdTokenClaims, IssuerUrl, Nonce, RedirectUrl, Scope};
use openidconnect::core::{CoreAuthenticationFlow, CoreClient, CoreGenderClaim, CoreProviderMetadata};
use retainer::{Cache, CacheExpiration};
use serde::{Deserialize, Serialize};
use tokio::select;
use tokio_util::sync::CancellationToken;

use crate::AppState;
use crate::error::AppError;
use crate::queries::mongo::MongoQueries;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub sub: String,
    pub email: String,
    pub dn: String,
    pub exp: usize,
    pub role: Role,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Role {
    USER,
    ADMIN,
}

pub struct RequireAdmin(pub User);

#[async_trait]
impl FromRequestParts<AppState> for User {
    type Rejection = AppError;

    async fn from_request_parts(request: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let Some(auth_header) = request.headers.get("Authorization") else { return Err(AppError::new(401, "unauthorized")); };
        let Ok(auth_header) = auth_header.to_str() else { return Err(AppError::new(400, "invalid auth header")); };

        if !auth_header.starts_with("Bearer ") {
            return Err(AppError::new(400, "invalid auth header"));
        }
        let auth_header = &auth_header[7..];

        let Ok(user) = state.jwt.validate_jwt(auth_header) else { return Err(AppError::new(400, "invalid auth header")); };

        Ok(user)
    }
}

#[async_trait]
impl FromRequestParts<AppState> for RequireAdmin {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let user = User::from_request_parts(parts, state).await?;

        if !matches!(user.role, Role::ADMIN) {
            return Err(AppError::new(403, "forbidden"));
        }

        Ok(RequireAdmin(user))
    }
}

impl Deref for RequireAdmin {
    type Target = User;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct JwtInstance {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    header: Header,
    validation: Validation,
}

impl JwtInstance {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_ref()),
            decoding_key: DecodingKey::from_secret(secret.as_ref()),
            header: Header::new(Algorithm::HS512),
            validation: Validation::new(Algorithm::HS512),
        }
    }

    pub fn create_jwt(&self, user: &User) -> anyhow::Result<String> {
        Ok(jsonwebtoken::encode(&self.header, &user, &self.encoding_key)?)
    }

    pub fn validate_jwt(&self, jwt: &str) -> anyhow::Result<User> {
        Ok(jsonwebtoken::decode::<User>(jwt, &self.decoding_key, &self.validation)?.claims)
    }
}

pub struct OpenIdInstance {
    client: CoreClient,
    sessions: Arc<Cache<String, String>>,
    cancel_token: CancellationToken,
}

pub type TokenClaims = IdTokenClaims<EmptyAdditionalClaims, CoreGenderClaim>;

impl OpenIdInstance {
    pub async fn new<S: Into<String>>(client_id: S, client_secret: S, issuer_url: S, hostname: S) -> anyhow::Result<Self> {
        let issuer_url = IssuerUrl::new(issuer_url.into())?;
        let metadata = CoreProviderMetadata::discover_async(issuer_url, openidconnect::reqwest::async_http_client).await?;

        let client = CoreClient::from_provider_metadata(metadata, ClientId::new(client_id.into()), Some(ClientSecret::new(client_secret.into())))
            .set_redirect_uri(RedirectUrl::new(hostname.into() + "/api/login/code")?);

        let cache = Arc::new(Cache::new());
        let cancel_token = CancellationToken::new();

        let cloned_cache = cache.clone();
        let cloned_cancel_token = cancel_token.clone();
        tokio::spawn(async move {
            select! {
               _ = cloned_cancel_token.cancelled() => {

               }
               _ = cloned_cache.monitor(1, 0.25, Duration::seconds(10).to_std().unwrap()) => {

               }
           }
        });

        Ok(
            Self {
                client,
                sessions: cache,
                cancel_token,
            }
        )
    }

    pub async fn create_auth_url(&self) -> String {
        let (url, csrf, nonce) = self.client.authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
            .add_scope(Scope::new("email".to_string()))
            .add_scope(Scope::new("profile".to_string()))
            .url();

        self.sessions.insert(csrf.secret().clone(), nonce.secret().clone(), CacheExpiration::from(60000)).await;

        url.to_string()
    }

    pub async fn fetch_token(&self, code: String, state: String) -> anyhow::Result<TokenClaims> {
        let nonce = self.sessions.get(&state).await;

        if nonce.is_none() {
            return Err(anyhow!("invalid state"));
        }

        let nonce = nonce.unwrap().clone();

        let res = self.client.exchange_code(AuthorizationCode::new(code))
            .request_async(openidconnect::reqwest::async_http_client).await
            .map_err(|e| anyhow!("invalid exchange code: {:?}", e))?;

        let token_verifier = self.client.id_token_verifier();
        let token_claims = res
            .extra_fields()
            .id_token().ok_or(anyhow!("server did not return an ID token"))?
            .claims(&token_verifier, &Nonce::new(nonce))?.clone();

        Ok(token_claims)
    }

    pub async fn user_from_claims(mongo: &MongoQueries, claims: TokenClaims) -> anyhow::Result<User> {
        let name = claims.name().ok_or(anyhow!("username empty"))?.get(None).ok_or(anyhow!("username locale invalid?"))?.to_string();
        let email = claims.email().ok_or(anyhow!("email empty"))?.to_string();
        let exp = (chrono::Local::now() + Duration::days(1)).timestamp() as usize;

        if !email.ends_with("@greenbear.berlin") && email != "zargor3@gmail.com" {
            return Err(anyhow!("third parties are not allowed to access"));
        }

        let role = mongo.get_user_role(&email).await?.unwrap_or(Role::USER);

        Ok(
            User {
                sub: name,
                email,
                dn: "greenBEAR".to_string(),
                exp,
                role,
            }
        )
    }
}

impl Drop for OpenIdInstance {
    fn drop(&mut self) {
        self.cancel_token.cancel();
    }
}
