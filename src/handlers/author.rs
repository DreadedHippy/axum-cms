use std::fmt::format;

use axum::Json;
use axum::extract::{Path, State};
use axum_extra::extract::WithRejection;

use crate::models::author::{Author, AuthorForCreate, AuthorForResult, AuthorForEdit};
use crate::models::custom_response::{CustomResponse, CustomResponseData};
use crate::models::error::{Result, Error};
use crate::models::state::AppState;
use crate::utils::auth::create_jwt;
use crate::utils::custom_extractor::ApiError;

// pub async fn handler_author_create(State(app_state): State<AppState>, Json(author_info): Json<AuthorForCreate>) -> Result<Json<CustomResponse<AuthorForResult>>> {
// 	println!("->> {:<12} - handler_author_create", "HANDLER");
// 	let author = app_state.create_author(author_info).await.map_err(|e| Error::CouldNotCreateAuthor)?;
// 	let jwt = create_jwt(author.email.clone())?;

// 	let response = CustomResponse::<AuthorForResult>::new(
// 		true,
// 		Some(format!("Author Created")),
// 		Some(CustomResponseData::Item(author))
// 	);

// 	Ok(Json(response))
// }

pub async fn handler_author_get_all(State(app_state): State<AppState>) -> Result<Json<CustomResponse<Author>>> {

	let authors = app_state.get_all_authors().await.map_err(|e|  Error::CouldNotGetAuthors)?;

	let response = CustomResponse::<Author>::new(
		true,
		Some(format!("Authors Retrieved")),
		Some(CustomResponseData::Collection(authors))
	);

	Ok(Json(response))
}

pub async fn handler_author_get_specific(State(app_state): State<AppState>, Path(id): Path<i64>) -> Result<Json<CustomResponse<Author>>> {

	let pool = app_state.pool;

	let author: Author = Author { id, name: format!("Resulting Author"), email: format!("result@mail.com"), password: format!("Password") };

	let response = CustomResponse::<Author>::new(
		true,
		Some(format!("Author Retrieved")),
		Some(CustomResponseData::Item(author))
	);

	Ok(Json(response))
}

pub async fn handler_author_edit(
	State(app_state): State<AppState>,
	Path(id): Path<i64>,
	WithRejection((Json(author)), _): WithRejection<Json<AuthorForEdit>, ApiError>
	) -> Result<Json<CustomResponse<Author>>> {
	let name = author.name.unwrap_or_default();

	let edited_author = app_state.edit_author(name, id).await.map_err(|e| {
		Error::CouldNotEditAuthor
	})?;

	let response = CustomResponse::<Author>::new(
		true,
		Some(format!("Author Updated successfully")),
		Some(CustomResponseData::Item(edited_author))
	);

	Ok(Json(response))
}

pub async fn handler_author_delete(State(app_state): State<AppState>, Path(id): Path<i64>) -> Result<Json<CustomResponse<Author>>> {
	// Delete the author, we don't care about the result, it only should throw no error
	let _ = app_state.delete_author(id).await.map_err(|e| {
		Error::CouldNotDeleteAuthor
	})?;

	let response = CustomResponse::<Author>::new(
		true,
		Some(format!("Author deleted successfully")),
		None
	);

	Ok(Json(response))
}