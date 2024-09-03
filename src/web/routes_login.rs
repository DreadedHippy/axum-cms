use crate::crypt::{pwd, EncryptContent};
use crate::ctx::Ctx;
use crate::models::author::{Author, AuthorBmc, AuthorForCreate, AuthorForLogin};
use crate::models::AppState;
use crate::web::custom_response::CustomResponse;
use crate::web::error::CrudError;
use crate::web::{self, remove_token_cookie, ServerError, ServerResult};
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use axum_extra::extract::WithRejection;
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

use super::auth::SignupPayload;
use super::{IncomingServerRequest, ServerResponse};

pub fn routes(app_state: AppState) -> Router {
	Router::new()
		.route("/api/signup", post(api_signup_handler))
		.route("/api/login", post(api_login_handler))
		.route("/api/logoff", post(api_logoff_handler))
		.with_state(app_state)
}

// region:    --- Login
async fn api_login_handler(
	State(app_state): State<AppState>,
	cookies: Cookies,
	Json(payload): Json<LoginPayload>,
) -> ServerResponse<()> {
	debug!("{:<12} - api_login_handler", "HANDLER");

	let LoginPayload {
		email,
		password: pwd_clear,
	} = payload;

	// -- Get the author.
	let author: AuthorForLogin = AuthorBmc::first_by_email(&app_state, &email)
		.await?
		.ok_or(ServerError::LoginFailEmailNotFound)?;
	let author_id = author.id;

	let ctx = Ctx::new(author_id);

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
	let body = Json(
		CustomResponse::<()>::new(
			true,
			Some("Logged in successfully".to_string()),
			None
		)
	);

	Ok(body)
}


#[derive(Debug, Deserialize)]
struct LoginPayload {
	email: String,
	password: String,
}
// endregion: --- Login

// region:    --- Signup
async fn api_signup_handler(
	State(app_state): State<AppState>,
	cookies: Cookies,
	WithRejection(Json(payload), _): IncomingServerRequest<SignupPayload>,
) -> ServerResponse<()> {
	debug!("{:<12} - api_signup_handler", "HANDLER");

	let SignupPayload {
		email,
		password: pwd_clear,
		name
	} = payload;

	let data = AuthorForCreate {
		name: name.clone(),
		email: email.clone(),
		password: pwd_clear.clone()
	};

	
	// -- Check for author in DB
	let author = AuthorBmc::first_by_email::<Author>(&app_state, &email).await?;

	// -- If author already exists, throw error
	if author.is_some() {
		return Err(ServerError::CreateFail(
			"Author".to_string(),
			"Author with the given email already exists".to_string(),
			CrudError::CONFLICT
		))
	}

	// -- Create author in db
	let author_id = AuthorBmc::create_no_auth(&app_state, data).await?;
	
	// -- Create context for new author
	let ctx = Ctx::new(author_id).map_err(|_| ServerError::SignupFail("Could not create new ctx for author".to_string()))?;

	// update author pwd
	AuthorBmc::update_pwd(&ctx, &app_state, author_id, &pwd_clear).await?;

	// Create the success body.
	let body = Json(
		CustomResponse::<()>::new(
			true,
			Some("Signed up successfully".to_string()),
			None
		)
	);
	

	Ok(body)
}

// endregion: --- Signup

// region:    --- Logoff
async fn api_logoff_handler(
	cookies: Cookies,
	Json(payload): Json<LogoffPayload>,
) -> ServerResponse<()> {
	debug!("{:<12} - api_logoff_handler", "HANDLER");
	let should_logoff = payload.logoff;

	if should_logoff {
		remove_token_cookie(&cookies)?;
	}

	// Create the success body.
	let body = Json(
		CustomResponse::<()>::new(
			true,
			Some("Logged off successfully".to_string()),
			None
		)
	);

	Ok(body)
}

#[derive(Debug, Deserialize)]
struct LogoffPayload {
	logoff: bool,
}
// endregion: --- Logoff