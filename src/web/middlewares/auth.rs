use axum::{ extract::{FromRequestParts, State}, http::{request::Parts, Request}, middleware::Next, response::Response};
use async_trait::async_trait;
use lazy_regex::regex_captures;
use tower_cookies::{Cookies, Cookie};
use tracing::debug;
use crate::{web::error::{ServerError, ServerResult}, models::state::AppState, utils::auth::is_jwt_valid};
use crate::web::middlewares::AUTHORIZATION_HEADER;
use crate::ctx::Ctx;

use super::AUTH_TOKEN;

pub async fn mw_ctx_resolver<B>(
	_mc: State<AppState>,
	cookies: Cookies,
	mut req:  Request<B>,
	next: Next<B>
) -> ServerResult<Response> {
	debug!("{:<12} - mw_ctx_resolver", "MIDDLEWARE");

	let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

	let result_ctx = match auth_token
		.ok_or(ServerError::AuthFailNoAuthTokenCookie)
		.and_then(parse_token)
	{
			Ok((user_id, exp, sign)) => {
				// TODO: Token components validation.
				Ok(Ctx::new(user_id))
			},
			Err(e) => {
				Err(e)
			}
	};

	// Remove the cookie if something went wrong other than no cookie
	if result_ctx.is_err() && matches!(result_ctx, Err(ServerError::AuthFailNoAuthTokenCookie)) {
		// cookies.remove(Cookie::named(AUTH_TOKEN))	
		cookies.remove(Cookie::named(AUTH_TOKEN))
	}

	// Store the ctx_result in the request extension.
	req.extensions_mut().insert(result_ctx);
	Ok(next.run(req).await)
}

/// Middleware to require authentication before accessing handler
pub async fn mw_require_auth<B>(
	ctx: ServerResult<Ctx>,
	mut req: Request<B>,
	next: Next<B>
) -> ServerResult<Response>{
	println!("{:<12} - mw_require_auth", "MIDDLEWARE");
	
	ctx?;

	Ok(next.run(req).await)
}
// 	// TODO: Real auth-token parsing & validation.
// 	let auth_token = auth_cookie.ok_or(ServerError::AuthFailNoAuthTokenCookie)?;
// 	let cookie_info = auth_token.split_whitespace().map(String::from).collect::<Vec<String>>();

// 	if cookie_info[0] != "Bearer" {
// 		return Err(ServerError::AuthFailNoAuthTokenCookie)
// 	}
	
// 	if let Some(jwt) = cookie_info.get(1) {
// 		is_jwt_valid(jwt)?;
// 	} else {
// 		return Err(ServerError::AuthFailNoAuthTokenCookie)
// 	}

// 	req.extensions_mut().insert(cookie_info[1].clone());

// 	Ok(next.run(req).await)
// }

/// Parse token of format `user-[user-id].expiration.[signature]`
/// Returns `(user_id, expiration, signature)`
fn parse_token(token: String) -> ServerResult<(i64, String, String)> {
	let (_whole, user_id, exp, sign) = regex_captures!(
		r#"^user-(\d+)\.(.+)\.(.+)"#, // Literal regex
		&token
	)
	.ok_or(ServerError::AuthFailTokenWrongFormat)?;

	let user_id = user_id.parse::<i64>().map_err(
		|_| ServerError::AuthFailTokenWrongFormat
	)?;

	Ok((user_id, exp.to_string(), sign.to_string()))
}



// region:    --- Ctx Extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
	type Rejection = ServerError;

	async fn from_request_parts(parts: &mut Parts, _state: &S) -> ServerResult<Self> {
		debug!("{:<12} - Ctx", "EXTRACTOR");

		parts
		.extensions
		.get::<ServerResult<Ctx>>()
		.ok_or(ServerError::AuthFailCtxNotInRequestExt)?
		.clone()
	
	}
}


// endregion: --- Ctx Extractor
