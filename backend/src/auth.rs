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
}

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

pub struct LdapInstance {
    pub url: String,
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
    pub fn new(url: &str, bind_cn: &str, password: &str, base_dn: &str) -> Self {
        Self {
            url: url.to_string(),
            bind_cn: bind_cn.to_string(),
            password: password.to_string(),
            base_dn: base_dn.to_string(),
        }
    }

    pub async fn search_user(&self, mail: &str) -> anyhow::Result<Option<LdapSearchResult>> {
        let (conn, mut ldap) = LdapConnAsync::new(&self.url).await?;
        ldap3::drive!(conn);
        ldap.simple_bind(&self.bind_cn, &self.password).await?.success()?;

        let Some(search) = ldap.search(&self.base_dn, Scope::Subtree,
                                       &format!("(mail={})", mail), vec!["dn", "cn"])
            .await?
            .success()?
            .0
            .into_iter()
            .map(SearchEntry::construct)
            .next() else { return Ok(None); };

        ldap.unbind().await?;

        Ok(Some(LdapSearchResult {
            dn: search.dn.to_string(),
            cn: search.attrs.get("cn").ok_or(anyhow!("attribute cn not found on {}", search.dn))?.first().unwrap().to_string(),
            email: mail.to_string(),
        }))
    }

    pub async fn check_password(&self, ldap_search: &LdapSearchResult, password: &str) -> anyhow::Result<bool> {
        let (conn, mut ldap) = LdapConnAsync::new(&self.url).await?;
        ldap3::drive!(conn);

        let is_authenticated = ldap.simple_bind(&ldap_search.dn, password).await?.success().is_ok();

        ldap.unbind().await?;
        Ok(is_authenticated)
    }


    pub async fn check_ldap_con(&self) -> anyhow::Result<()> {
        let (conn, mut ldap) = LdapConnAsync::new(&self.url).await?;
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
        }
    }
}
