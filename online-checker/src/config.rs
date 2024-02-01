use serde::{Deserialize, Serialize};

use crate::{Error, Result};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub web_addr: String,
    pub jwt_secret_key: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        config::Config::builder()
            .add_source(config::Environment::default())
            .build()
            .map_err(Error::from)?
            .try_deserialize()
            .map_err(Error::from)
    }
}
