use axum::{response::IntoResponse, http::StatusCode, extract::rejection::JsonRejection, Json};
use serde::Serialize;
use serde_json::{json, Value};
use tracing::debug;

use crate::{crypt, models, web};
pub type ServerResult<T> = core::result::Result<T, ServerError>;
use super::*;

/// Custom error enum, holding errors encountered during request handling
#[derive(Debug, strum_macros::AsRefStr, Serialize)]
pub enum ServerError {

	LoginFail,
	// -- Login
	LoginFailEmailNotFound,
	LoginFailAuthorHasNoPwd {author_id: i64},
	LoginFailPwdNotMatching {author_id: i64},

	// -- CtxExtError
	CtxExt(middlewares::auth::CtxExtError),

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

	// -- Modules
	Model(models::ModelError),
	Crypt(crypt::CryptError),

}


// region:    --- Froms
impl From<models::ModelError> for ServerError {
	fn from(val: models::ModelError) -> Self {
		ServerError::Model(val)
	}
}

impl From<crypt::CryptError> for ServerError {
	fn from(val: crypt::CryptError) -> Self {
		Self::Crypt(val)
	}
}
// endregion: --- Froms

impl IntoResponse for ServerError {
	fn into_response(self) -> axum::response::Response {
		debug!(" {:<12} - {self:?}", "INTO_RES");

		// Create a placeholder Axum reponse.
		let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

		// Insert the Error into the reponse.
		response.extensions_mut().insert(self);

		response
	}
}

/// From the root error to the http status code and ClientError
impl ServerError {
	pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
		use web::ServerError::*;

		#[allow(unreachable_patterns)]
		match self {
			// -- Login
			LoginFailEmailNotFound
			| LoginFailAuthorHasNoPwd { .. }
			| LoginFailPwdNotMatching { .. } => {
				(StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL)
			},

			// -- Auth
			CtxExt(_) => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

			// -- Fallback.
			_ => (
				StatusCode::INTERNAL_SERVER_ERROR,
				ClientError::SERVICE_ERROR,
			),
		}
	}
}


// region:    --- ServerError Boilerplate
impl core::fmt::Display for ServerError {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for ServerError {}
// endregion: --- ServerError Boilerplate


#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
	LOGIN_FAIL,
	NO_AUTH,
	SERVICE_ERROR,
}
// endregion: --- Client Error