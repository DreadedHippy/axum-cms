use axum::{response::IntoResponse, http::StatusCode};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	LoginFail, // Due to invalid credentials

	// -- Auth errors.
	AuthFailNoAuthTokenCookie,
	AuthFailCookieExpired,
	InvalidJwt,

	// -- Model errors.
	
	// Author
	CouldNotCreateAuthor,
	CouldNotGetAuthors,
	CouldNotGetAuthor,

	// Post
	CouldNotCreatePost,
	CouldNotGetPosts,
	CouldNotGetPost,

	// Server
	InternalServerError

	// -- 
}

impl IntoResponse for Error {
	fn into_response(self) -> axum::response::Response {
		println!("->> {:<12} - {self:?}", "INTO_RES");

		match self {
			// Unauthenticated
			Error::AuthFailCookieExpired => (StatusCode::UNAUTHORIZED, "AUTH_FAILED_COOKIE_EXPIRED").into_response(),
			Error::AuthFailNoAuthTokenCookie => (StatusCode::UNAUTHORIZED, "AUTH_FAILED_NO_AUTH_TOKEN_COOKIE").into_response(),
			Error::InvalidJwt => (StatusCode::UNAUTHORIZED, "INVALID_JWT").into_response(),


			Error::LoginFail => (StatusCode::BAD_REQUEST, "LOGIN_FAILED_INVALID_CREDENTIALS").into_response(),
			Error::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response(),
			_ => (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
		}
	}
}