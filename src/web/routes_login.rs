use crate::crypt::{pwd, EncryptContent};
use crate::ctx::Ctx;
use crate::models::author::{AuthorBmc, AuthorForLogin};
use crate::models::state::AppState;
use crate::web::{self, remove_token_cookie, ServerError, ServerResult};
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

pub fn routes(mm: AppState) -> Router {
	Router::new()
		.route("/api/login", post(api_login_handler))
		.route("/api/logoff", post(api_logoff_handler))
		.with_state(mm)
}

// region:    --- Login
async fn api_login_handler(
	State(app_state): State<AppState>,
	cookies: Cookies,
	Json(payload): Json<LoginPayload>,
) -> ServerResult<Json<Value>> {
	debug!("{:<12} - api_login_handler", "HANDLER");

	let LoginPayload {
		email,
		password: pwd_clear,
	} = payload;
	let root_ctx = Ctx::root_ctx();

	// -- Get the author.
	let author: AuthorForLogin = AuthorBmc::first_by_email(&root_ctx, &app_state, &email)
		.await?
		.ok_or(ServerError::LoginFailEmailNotFound)?;
	let author_id = author.id;

	// -- Validate the password.
	let Some(pwd) = author.password else {
		return Err(ServerError::LoginFailAuthorHasNoPwd{ author_id });
	};

	pwd::validate_pwd(
		&EncryptContent {
			salt: author.password_salt.to_string(),
			content: pwd_clear.clone(),
		},
		&pwd,
	)
	.map_err(|_| ServerError::LoginFailPwdNotMatching { author_id })?;

	// -- Set web token.
	web::set_token_cookie(&cookies, &author.email, &author.token_salt.to_string())?;

	// Create the success body.
	let body = Json(json!({
		"result": {
			"success": true
		}
	}));

	Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
	email: String,
	password: String,
}
// endregion: --- Login

// region:    --- Logoff
async fn api_logoff_handler(
	cookies: Cookies,
	Json(payload): Json<LogoffPayload>,
) -> ServerResult<Json<Value>> {
	debug!("{:<12} - api_logoff_handler", "HANDLER");
	let should_logoff = payload.logoff;

	if should_logoff {
		remove_token_cookie(&cookies)?;
	}

	// Create the success body.
	let body = Json(json!({
		"result": {
			"logged_off": should_logoff
		}
	}));

	Ok(body)
}

#[derive(Debug, Deserialize)]
struct LogoffPayload {
	logoff: bool,
}
// endregion: --- Logoff