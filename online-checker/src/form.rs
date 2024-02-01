use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
    pub exp_mins: u8,
}
