use axum::{response::IntoResponse, http::StatusCode, extract::rejection::JsonRejection, Json};
use serde_json::{json, Value};
use tracing::debug;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]

/// Custom error enum, holding errors encountered during request handling
pub enum Error {
	LoginFail, // Due to invalid credentials

	// region: -- Auth errors.

	/// Auth failed, no auth token in cookie
	AuthFailNoAuthTokenCookie,
	/// Auth failed, auth token cookie exists but is expired
	AuthFailCookieExpired,
	/// Auth failed, JWT is invalid
	InvalidJwt,

	
	// endregion: -- Auth errors.

	// region: -- Model errors.
	
	// Author

	/// Author could not be created
	CouldNotCreateAuthor,
	/// List of authors could not be gotten
	CouldNotGetAuthors,
	/// A specific author could not be retrieved
	CouldNotGetAuthor,
	/// A specific author could not be edited
	CouldNotEditAuthor,
	/// A specific author could not be deleted
	CouldNotDeleteAuthor,

	// Post

	/// Post could not be created
	CouldNotCreatePost,
	/// List of posts could not be retrieved
	CouldNotGetPosts,
	/// A post could not be retrieved
	CouldNotGetPost,
	/// A post could not be edited
	CouldNotEditPost,
	/// A post could not be deleted
	CouldNotDeletePost,
	/// A post can only be edited by its own author
	OnlyAuthorCanEdit,

	// endregion: -- Model errors

	// region: -- Misc.
	// Server
	/// An error occurred on the server, not sure which
	InternalServerError,

	// Cache
	/// Something happened and Redis failed to connect
	CouldNotConnectToRedis,
	// CouldNotFetchPosts,

	// -- Config
	ConfigMissingEnv(&'static str)

	// endregion: -- Misc.
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

			Error::OnlyAuthorCanEdit => {
				let payload = json!({
					"status": false,
					"message": "Only the author of this post may edit it"
				});
				(StatusCode::FORBIDDEN, Json(payload)).into_response()
			}
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