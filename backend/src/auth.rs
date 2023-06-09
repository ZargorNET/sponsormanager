use std::ops::Deref;

use anyhow::anyhow;
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use chrono::Duration;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use ldap3::{LdapConnAsync, Scope, SearchEntry};
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::error::AppError;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub sub: String,
    pub email: String,
    pub dn: String,
    pub exp: usize,
    pub role: Role,
}

#[derive(Serialize, Deserialize, Debug)]
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
            validation: Validation::new(Algorithm::HS512)
        }
    }

    pub fn create_jwt(&self, user: &User) -> anyhow::Result<String> {
        Ok(jsonwebtoken::encode(&self.header, &user, &self.encoding_key)?)
    }

    pub fn validate_jwt(&self, jwt: &str) -> anyhow::Result<User> {
        Ok(jsonwebtoken::decode::<User>(jwt, &self.decoding_key, &self.validation)?.claims)
    }
}

pub struct LdapInstance {
    pub uri: String,
    pub bind_cn: String,
    pub password: String,
    pub base_dn: String,
}

#[derive(Debug, Clone)]
pub struct LdapSearchResult {
    pub dn: String,
    pub cn: String,
    pub email: String,
}

impl LdapInstance {
    pub fn new(uri: &str, bind_cn: &str, password: &str, base_dn: &str) -> Self {
        Self {
            uri: uri.to_string(),
            bind_cn: bind_cn.to_string(),
            password: password.to_string(),
            base_dn: base_dn.to_string(),
        }
    }

    pub async fn search_user(&self, mail: &str) -> anyhow::Result<Option<LdapSearchResult>> {
        let (conn, mut ldap) = LdapConnAsync::new(&self.uri).await?;
        ldap3::drive!(conn);
        ldap.simple_bind(&self.bind_cn, &self.password).await?.success()?;

        let search = ldap.search(&self.base_dn, Scope::Subtree,
                                 &format!("(mail={})", mail), vec!["dn", "cn"])
            .await?
            .success()?
            .0
            .into_iter()
            .map(SearchEntry::construct)
            .next();

        ldap.unbind().await?;

        let Some(search) = search  else { return Ok(None); };

        Ok(Some(LdapSearchResult {
            dn: search.dn.to_string(),
            cn: search.attrs.get("cn").ok_or(anyhow!("attribute cn not found on {}", search.dn))?.first().unwrap().to_string(),
            email: mail.to_string(),
        }))
    }

    pub async fn check_password(&self, ldap_search: &LdapSearchResult, password: &str) -> anyhow::Result<bool> {
        let (conn, mut ldap) = LdapConnAsync::new(&self.uri).await?;
        ldap3::drive!(conn);

        let is_authenticated = ldap.simple_bind(&ldap_search.dn, password).await?.success().is_ok();

        ldap.unbind().await?;
        Ok(is_authenticated)
    }


    pub async fn check_ldap_con(&self) -> anyhow::Result<()> {
        let (conn, mut ldap) = LdapConnAsync::new(&self.uri).await?;
        ldap3::drive!(conn);
        ldap.simple_bind(&self.bind_cn, &self.password).await?.success()?;
        ldap.unbind().await?;
        Ok(())
    }
}

impl From<LdapSearchResult> for User {
    fn from(value: LdapSearchResult) -> Self {
        Self {
            sub: value.cn,
            email: value.email,
            dn: value.dn,
            exp: (chrono::Local::now() + Duration::days(1)).timestamp() as usize,
            role: Role::USER
        }
    }
}
