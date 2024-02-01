use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthBody {
    pub token: String,
    pub token_type: String,
}

impl AuthBody {
    pub fn new(token: String) -> Self {
        Self {
            token,
            token_type: "Bearer".to_string(),
        }
    }
}
