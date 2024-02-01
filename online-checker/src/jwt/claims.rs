use jsonwebtoken::{decode, encode, Header, Validation};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::{AuthBody, Key};
use crate::{Error, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims<T> {
    /// 主题
    pub sub: String,
    /// 签发机构
    pub iss: String,
    /// 过期时间
    pub exp: i64,
    /// 签发时间
    pub iat: i64,
    /// 数据
    pub data: T,
}

impl<T: Serialize + DeserializeOwned> Claims<T> {
    pub fn new(exp: i64, iat: i64, data: T) -> Self {
        Self {
            sub: "ONLINE-CHECKER".to_string(),
            iss: "axum.rs".to_string(),
            exp,
            iat,
            data,
        }
    }

    pub fn with_exp(exp_mins: u32, data: T) -> Self {
        let now = chrono::Utc::now();
        let exp = now + chrono::Duration::minutes(exp_mins as i64);

        let iat = now.timestamp();
        let exp = exp.timestamp();

        Self::new(exp, iat, data)
    }

    pub fn token(&self, key: &Key) -> Result<AuthBody> {
        let token = encode(&Header::default(), &self, &key.encoding).map_err(Error::from)?;
        Ok(AuthBody::new(token))
    }

    pub fn from_token(token: &str, key: &Key) -> Result<Self> {
        let token_data =
            decode(token, &key.decoding, &Validation::default()).map_err(Error::from)?;
        Ok(token_data.claims)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserClaimsData {
    pub id: String,
    pub email: String,
}
