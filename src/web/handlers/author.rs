use std::fmt::format;

use axum::{debug_handler, Extension, Json};
use axum::extract::{Path, State};
use axum_extra::extract::WithRejection;
use tracing::{debug, info};

use crate::ctx::Ctx;
use crate::models::author::{Author, AuthorBmc, AuthorForCreate, AuthorForEdit, AuthorForResult};
use crate::web::custom_response::{CustomResponse, CustomResponseData};
use crate::web::error::{ServerResult, ServerError};
use crate::web::custom_extractor::ApiError;
use crate::models::state::AppState;
use crate::utils::auth::{create_jwt, get_info_from_jwt};
use crate::web::{IncomingServerRequest, ServerResponse};

pub async fn handler_author_create(
	State(app_state): State<AppState>,
	WithRejection(Json(data), _): IncomingServerRequest<AuthorForCreate>
) -> ServerResponse<AuthorForResult> {
	debug!(" {:<12} - handler_author_create", "HANDLER");

	let id = AuthorBmc::create_no_auth(&app_state, data).await?;

	let author: AuthorForResult = AuthorBmc::get_no_auth(&app_state, id).await?;

	let response = CustomResponse::<AuthorForResult>::new(
		true,
		Some(format!("Author Created")),
		Some(CustomResponseData::Item(author))
	);

	Ok(Json(response))
}

/// Handler to list all authors
pub async fn handler_author_list(State(app_state): State<AppState>) -> ServerResponse<AuthorForResult> {
	debug!("{:>12} - handler_author", "HANDLER");

	let authors = AuthorBmc::list(&app_state).await?;
	// let authors = app_state.get_all_authors().await.map_err(|e|  ServerError::CouldNotGetAuthors)?;
	let authors = authors.into_iter().map(AuthorForResult::from).collect::<Vec<_>>();

	let response = CustomResponse::<AuthorForResult>::new(
		true,
		Some(format!("Authors Retrieved")),
		Some(CustomResponseData::Collection(authors))
	);

	Ok(Json(response))
}

/// Handler to get an author
pub async fn handler_author_get(State(app_state): State<AppState>, Path(id): Path<i64>) -> ServerResponse<AuthorForResult> {
	debug!("{:>12} - handler_author", "HANDLER");

	let author: AuthorForResult = AuthorBmc::get_no_auth(&app_state, id).await?;

	let response = CustomResponse::new(
		true,
		Some(format!("Author Retrieved")),
		Some(CustomResponseData::Item(author))
	);

	Ok(Json(response))
}

/// Handler to edit a specific author
#[debug_handler]
pub async fn handler_author_edit(
	State(app_state): State<AppState>,
	ctx: Ctx,
	Extension(token): Extension<String>,
	Path(id): Path<i64>,
	WithRejection((Json(author)), _): WithRejection<Json<AuthorForEdit>, ApiError>
	) -> ServerResult<Json<CustomResponse<Author>>> {
	let name = author.name.unwrap_or_default();

	let (_, author_id) = get_info_from_jwt(token)?;

	if author_id != id {
		return Err(ServerError::OnlyAuthorCanEditSelf)
	}

	let edited_author = app_state.edit_author(name, id).await.map_err(|e| {
		ServerError::CouldNotEditAuthor
	})?;

	let response = CustomResponse::<Author>::new(
		true,
		Some(format!("Author Updated successfully")),
		Some(CustomResponseData::Item(edited_author))
	);

	Ok(Json(response))
}

/// Handler to delete an author
pub async fn handler_author_delete(State(app_state): State<AppState>, Path(id): Path<i64>) -> ServerResult<Json<CustomResponse<Author>>> {
	// Delete the author, we don't care about the result, it only should throw no error
	let _ = app_state.delete_author(id).await.map_err(|e| {
		ServerError::CouldNotDeleteAuthor
	})?;

	let response = CustomResponse::<Author>::new(
		true,
		Some(format!("Author deleted successfully")),
		None
	);

	Ok(Json(response))
}