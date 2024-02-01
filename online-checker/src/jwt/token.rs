use serde::{de::DeserializeOwned, Serialize};

use crate::Result;

use super::{AuthBody, Claims, Key};

pub fn encode<T>(secret: &str, exp_mins: u32, data: T) -> Result<AuthBody>
where
    T: Serialize + DeserializeOwned,
{
    let key = Key::with(secret);
    let claims = Claims::with_exp(exp_mins, data);
    claims.token(&key)
}

pub fn decode<T>(token: &str, secret: &str) -> Result<Claims<T>>
where
    T: Serialize + DeserializeOwned,
{
    let key = Key::with(secret);
    Claims::<T>::from_token(token, &key)
}
