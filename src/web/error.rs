use axum::{response::IntoResponse, http::StatusCode, extract::rejection::JsonRejection, Json};
use serde::Serialize;
use serde_json::{json, Value};
use tracing::debug;

use crate::models;
pub type ServerResult<T> = core::result::Result<T, ServerError>;

/// Custom error enum, holding errors encountered during request handling
#[derive(Debug, Clone, strum_macros::AsRefStr, Serialize)]
pub enum ServerError {

	LoginFail, // Due to invalid credentials

	// region: -- Auth errors.

	/// Auth failed, no auth token in cookie
	AuthFailNoAuthTokenCookie,
	/// Auth failed, auth token cookie exists but is expired
	AuthFailCookieExpired,
	/// Auth failed, auth token cookie is in the wrong format
	AuthFailTokenWrongFormat,
	/// Auth failed, context is not in request extension
	AuthFailCtxNotInRequestExt,
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
	/// Only an author can edit their own name
	OnlyAuthorCanEditSelf,
	/// Author with a corresponding email already exists
	AuthorAlreadyExists,

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

	/// Edit Suggestion could not be created
	CouldNotCreateEditSuggestion,

	// endregion: -- Model errors

	// region: -- Misc.
	// Server
	/// An error occurred on the server, not sure which
	InternalServerError,

	// Cache
	/// Something happened and Redis failed to connect
	CouldNotConnectToRedis,
	// CouldNotFetchPosts,

	// endregion: -- Misc.
}

impl IntoResponse for ServerError {
	fn into_response(self) -> axum::response::Response {
		debug!(" {:<12} - {self:?}", "INTO_RES");

		match self {
			// region: -- Unauthenticated
			ServerError::AuthFailCookieExpired => {
				let payload = json!({
					"status": false,
					"message": "Auth token expired"
				});
				(StatusCode::UNAUTHORIZED, Json(payload)).into_response()
			},

			ServerError::AuthFailNoAuthTokenCookie => {
				let payload = json!({
					"status": false,
					"message": "No auth token"
				});
				(StatusCode::UNAUTHORIZED, Json(payload)).into_response()
			},

			ServerError::InvalidJwt => {
				let payload = json!({
					"status": false,
					"message": "Invalid JWT"
				});
				(StatusCode::UNAUTHORIZED, Json(payload)).into_response()
			},
			// endregion: -- Unauthenticated

			// region: -- Not found
			ServerError::CouldNotGetPost => {
				let payload = json!({
					"status": false,
					"message": "Post not found"
				});
				(StatusCode::NOT_FOUND, Json(payload)).into_response()
			},

			ServerError::CouldNotGetAuthor => {
				let payload = json!({
					"status": false,
					"message": "Author not found"
				});

				(StatusCode::NOT_FOUND, Json(payload)).into_response()
			},
			// endregion: -- Not found

			ServerError::LoginFail => {
				let payload = json!({
					"status": false,
					"message": "Login failed, Invalid credentials"
				});
				(StatusCode::BAD_REQUEST,  Json(payload)).into_response()
			},

			ServerError::AuthorAlreadyExists => {
				let payload = json!({
					"status": false,
					"message": "Signup failed, this email is already registered"
				});
				(StatusCode::BAD_REQUEST,  Json(payload)).into_response()
			},

			ServerError::InternalServerError => {
				let payload = json!({
					"status": false,
					"message": "An error occurred on the server"
				});
				(StatusCode::INTERNAL_SERVER_ERROR, Json(payload)).into_response()
			},

			ServerError::OnlyAuthorCanEdit => {
				let payload = json!({
					"status": false,
					"message": "Only the author of this post may edit it"
				});
				(StatusCode::FORBIDDEN, Json(payload)).into_response()
			},
			

			ServerError::OnlyAuthorCanEditSelf => {
				let payload = json!({
					"status": false,
					"message": "Please login to the account to edit"
				});
				(StatusCode::UNAUTHORIZED, Json(payload)).into_response()
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


impl ServerError {
	pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
		#[allow(unreachable_patterns)]
		match self {
			Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

			// -- Auth
			Self::AuthFailNoAuthTokenCookie
			| Self::AuthFailTokenWrongFormat
			| Self::AuthFailCtxNotInRequestExt => {
				(StatusCode::FORBIDDEN, ClientError::NO_AUTH)
			},
			// -- Fallback
			_ => (
				StatusCode::INTERNAL_SERVER_ERROR,
				ClientError::SERVICE_ERROR
			)
		}
	}
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
	LOGIN_FAIL,
	NO_AUTH,
	SERVICE_ERROR,
}
// endregion: --- Client Error