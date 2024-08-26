pub use self::error::ClientError;
pub use self::error::{ServerError, ServerResult};
use crate::crypt::token::generate_web_token;
use axum::Json;
use axum_extra::extract::WithRejection;
use custom_extractor::ApiError;
use custom_response::CustomResponse;
use tower_cookies::{Cookie, Cookies};

pub mod handlers;
pub mod middlewares;
pub mod routes;
mod custom_extractor;
mod error;
pub mod custom_response;
pub mod auth;
pub mod routes_login;

type ServerResponse<T> = ServerResult<Json<CustomResponse<T>>>;
type IncomingServerRequest<T> =  WithRejection<Json<T>, ApiError>;

#[derive(Debug)]
pub struct HelloParams {
	name: Option<String>
}

pub const AUTH_TOKEN: &str = "auth-token";

// endregion: --- Modules


fn set_token_cookie(cookies: &Cookies, user: &str, salt: &str) -> ServerResult<()> {
	let token = generate_web_token(user, salt)?;

	let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());
	cookie.set_http_only(true); // prevent client-side script access
	cookie.set_path("/"); // set valid domain to server rot

	cookies.add(cookie);

	Ok(())
}

fn remove_token_cookie(cookies: &Cookies) -> ServerResult<()> {
	let mut cookie = Cookie::named(AUTH_TOKEN);
	cookie.set_path("/");

	cookies.remove(cookie);

	Ok(())
}