use axum::{response::IntoResponse, http::StatusCode, extract::rejection::JsonRejection};
use tracing::debug;

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
	CouldNotEditAuthor,
	CouldNotDeleteAuthor,

	// Post
	CouldNotCreatePost,
	CouldNotGetPosts,
	CouldNotGetPost,
	CouldNotEditPost,
	CouldNotDeletePost,

	// Server
	InternalServerError,

	// Cache
	CouldNotConnectToRedis,
	// CouldNotFetchPosts,

	// -- Config
	ConfigMissingEnv(&'static str)

	// -- 
}

impl IntoResponse for Error {
	fn into_response(self) -> axum::response::Response {
		debug!(" {:<12} - {self:?}", "INTO_RES");

		match self {
			// Unauthenticated
			Error::AuthFailCookieExpired => (StatusCode::UNAUTHORIZED, "AUTH_FAILED_COOKIE_EXPIRED").into_response(),
			Error::AuthFailNoAuthTokenCookie => (StatusCode::UNAUTHORIZED, "AUTH_FAILED_NO_AUTH_TOKEN_COOKIE").into_response(),
			Error::InvalidJwt => (StatusCode::UNAUTHORIZED, "INVALID_JWT").into_response(),

			// Not found
			Error::CouldNotGetPost => (StatusCode::NOT_FOUND, "POST_NOT_FOUND").into_response(),
			Error::CouldNotGetAuthor => (StatusCode::NOT_FOUND, "AUTHOR_NOT_FOUND").into_response(),

			Error::LoginFail => (StatusCode::BAD_REQUEST, "LOGIN_FAILED_INVALID_CREDENTIALS").into_response(),
			Error::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response(),
			_ => (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
		}
	}
}