use jsonwebtoken::{DecodingKey, EncodingKey};

pub struct Key {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Key {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }

    pub fn with(secret: &str) -> Self {
        Self::new(secret.as_bytes())
    }
}
