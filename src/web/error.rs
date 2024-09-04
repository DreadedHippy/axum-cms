use axum::{response::IntoResponse, http::StatusCode, extract::rejection::JsonRejection, Json};
use serde::Serialize;
use serde_json::{json, Value};
use tracing::debug;
use std::string::ToString;

use crate::{crypt, models, web};
pub type ServerResult<T> = core::result::Result<T, ServerError>;
use super::*;

/// Custom error enum, holding errors encountered during request handling
#[derive(Debug, strum_macros::AsRefStr, Serialize)]
pub enum ServerError {

	// -- Signup
	/// SignupFail(`Reason`)
	SignupFail(String),

	// -- Login
	LoginFail,
	LoginFailEmailNotFound,
	LoginFailAuthorHasNoPwd {author_id: i64},
	LoginFailPwdNotMatching {author_id: i64},

	// -- CtxExtError
	CtxExt(middlewares::auth::CtxExtError),

	// -- CRUD errors
	///? CRUD Type (Model, Reason, Status code)
	CreateFail(String, String, CrudError),
	///? CRUD Type (Model, Reason, Status code)
	ListFail(String, String, CrudError),
	///? CRUD Type (Model, Reason, Status code)
	GetFail(String, String, CrudError),
	///? CRUD Type (`Model`, `Reason`, `Status code`)
	UpdateFail(String, String, CrudError),
	///? CRUD Type (Model, Reason, Status code)
	DeleteFail(String, String, CrudError),

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
	SerdeJson(String),

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

impl From<serde_json::Error> for ServerError {
	fn from(val: serde_json::Error) -> Self {
		Self::SerdeJson(val.to_string())
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

		// #[allow(unreachable_patterns)]
		match self {

			// -- Signup
			SignupFail(_) => {
				(StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVICE_ERROR)
			},


			// -- Login
			LoginFailEmailNotFound
			| LoginFailAuthorHasNoPwd { .. }
			| LoginFailPwdNotMatching { .. } => {
				(StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL)
			},

			// -- Auth
			CtxExt(_) => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

			// -- Crud
			CreateFail(model_name, reason, crud_error) => {
				let status_code: StatusCode = crud_error.into();
				let error_message = format!("{} create failed, {}", model_name, reason);

				(status_code, ClientError::CUSTOM(error_message))
			},
			
			ListFail(model_name, reason, crud_error) => {
				let status_code: StatusCode = crud_error.into();
				let error_message = format!("{}s list failed, {}", model_name, reason);

				(status_code, ClientError::CUSTOM(error_message))
			},
			
			GetFail(model_name, reason, crud_error) => {
				let status_code: StatusCode = crud_error.into();
				let error_message = format!("{} get failed, {}", model_name, reason);

				(status_code, ClientError::CUSTOM(error_message))
			},
			
			UpdateFail(model_name, reason, crud_error) => {
				let status_code: StatusCode = crud_error.into();
				let error_message = format!("{} update failed, {}", model_name, reason);

				(status_code, ClientError::CUSTOM(error_message))
			},
			
			DeleteFail(model_name, reason, crud_error) => {
				let status_code: StatusCode = crud_error.into();
				let error_message = format!("{} delete failed, {}", model_name, reason);

				(status_code, ClientError::CUSTOM(error_message))
			},

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


// region:   --- Client Error

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
	LOGIN_FAIL,
	NO_AUTH,
	SERVICE_ERROR,
	// #[strum(to_string = "{0}")]
	CUSTOM(String)
}

// endregion: --- Client Error

// region:    --- CRUD Error codes
#[derive(Debug, strum_macros::AsRefStr, Serialize)]
#[allow(non_camel_case_types)]
pub enum CrudError {
	FORBIDDEN,
	BAD_REQUEST,
	UNAUTHORIZED,
	CONFLICT,
	INTERNAL_SERVER_ERROR
}

impl From<&CrudError> for StatusCode {
	fn from(value: &CrudError) -> Self {
		match value {
			CrudError::BAD_REQUEST => StatusCode::BAD_REQUEST,
			CrudError::FORBIDDEN => StatusCode::FORBIDDEN,
			CrudError::UNAUTHORIZED => StatusCode::UNAUTHORIZED,
			CrudError::CONFLICT => StatusCode::CONFLICT,
			CrudError::INTERNAL_SERVER_ERROR => StatusCode::INTERNAL_SERVER_ERROR
		}
	}
}
// endregion: --- CRUD Error codes