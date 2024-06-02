use axum::{http::Request, middleware::Next, response::Response};
use tower_cookies::Cookies;
use tracing::debug;
use crate::{models::error::{ServerResult, ServerError}, utils::auth::is_jwt_valid};
use crate::web::middlewares::AUTHORIZATION_HEADER;

use super::AUTH_TOKEN;

/// Middleware to require authentication before accessing handler
pub async fn mw_require_auth<B>(
	cookies: Cookies,
	mut req: Request<B>,
	next: Next<B>
) -> ServerResult<Response> {
	debug!(" {:<12} - mw_require_auth", "MIDDLEWARE");
	let auth_cookie = cookies.get(AUTHORIZATION_HEADER).map(|c| c.value().to_string());

	// TODO: Real auth-token parsing & validation.
	let auth_token = auth_cookie.ok_or(ServerError::AuthFailNoAuthTokenCookie)?;
	let cookie_info = auth_token.split_whitespace().map(String::from).collect::<Vec<String>>();

	if cookie_info[0] != "Bearer" {
		return Err(ServerError::AuthFailNoAuthTokenCookie)
	}
	
	if let Some(jwt) = cookie_info.get(1) {
		is_jwt_valid(jwt)?;
	} else {
		return Err(ServerError::AuthFailNoAuthTokenCookie)
	}

	req.extensions_mut().insert(cookie_info[1].clone());

	Ok(next.run(req).await)
}

