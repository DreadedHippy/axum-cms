use axum::{response::IntoResponse, http::StatusCode, extract::rejection::JsonRejection, Json};
use serde_json::{json, Value};
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
			// region: -- Unauthenticated
			Error::AuthFailCookieExpired => {
				let payload = json!({
					"status": false,
					"message": "Auth token expired"
				});
				(StatusCode::UNAUTHORIZED, Json(payload)).into_response()
			},

			Error::AuthFailNoAuthTokenCookie => {
				let payload = json!({
					"status": false,
					"message": "No auth token"
				});
				(StatusCode::UNAUTHORIZED, Json(payload)).into_response()
			},

			Error::InvalidJwt => {
				let payload = json!({
					"status": false,
					"message": "Invalid JWT"
				});
				(StatusCode::UNAUTHORIZED, Json(payload)).into_response()
			},
			// endregion: -- Unauthenticated

			// region: -- Not found
			Error::CouldNotGetPost => {
				let payload = json!({
					"status": false,
					"message": "Post not found"
				});
				(StatusCode::NOT_FOUND, Json(payload)).into_response()
			},

			Error::CouldNotGetAuthor => {
				let payload = json!({
					"status": false,
					"message": "Author not found"
				});

				(StatusCode::NOT_FOUND, Json(payload)).into_response()
			},
			// endregion: -- Not found

			Error::LoginFail => {
				let payload = json!({
					"status": false,
					"message": "Login failed, Invalid credentials"
				});
				(StatusCode::BAD_REQUEST,  Json(payload)).into_response()
			},

			Error::InternalServerError => {
				let payload = json!({
					"status": false,
					"message": "An error occurred on the server"
				});
				(StatusCode::INTERNAL_SERVER_ERROR, Json(payload)).into_response()
			},
			_ => {
				let payload = json!({
					"status": false,
					"message": "An error occurred on the server"
				});
				(StatusCode::INTERNAL_SERVER_ERROR, Json(payload)).into_response()
			}
		}
	}
}