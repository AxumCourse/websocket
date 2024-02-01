use std::sync::Arc;

use axum::{extract::State, Json};

use crate::{form::LoginForm, jwt, resp::JsonResp, AppState, Error, Result};

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(frm): Json<LoginForm>,
) -> Result<Json<JsonResp<jwt::AuthBody>>> {
    if !(frm.email == "team@axum.rs" && frm.password == "axum.rs") {
        return Err(Error::invalid_parameter("用户名或密码错误"));
    }

    let ucd = jwt::UserClaimsData {
        id: xid::new().to_string(),
        email: frm.email,
    };

    let ab = jwt::token::encode(&state.cfg.jwt_secret_key, frm.exp_mins as u32, ucd)?;

    Ok(Json(JsonResp::ok(ab)))
}
pub async fn logout() -> Result<Json<String>> {
    unimplemented!()
}
