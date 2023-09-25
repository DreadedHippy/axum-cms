use std::fmt::format;

use axum::Json;
use axum::extract::{Path, State};

use crate::models::author::{Author, AuthorForCreate};
use crate::models::custom_response::{CustomResponse, CustomResponseData};
use crate::models::error::{Result, Error};
use crate::models::state::AppState;

pub async fn handler_author_create(State(app_state): State<AppState>, Json(author_info): Json<AuthorForCreate>) -> Result<Json<CustomResponse<Author>>> {
	let author = app_state.create_author(author_info).await.map_err(|e| Error::CouldNotCreateAuthor)?;

	let response = CustomResponse::<Author>::new(
		true,
		Some(format!("Author Created")),
		Some(CustomResponseData::Item(author))
	);

	Ok(Json(response))
}

pub async fn handler_author_get_all(State(app_state): State<AppState>) -> Result<Json<CustomResponse<Author>>> {
	let random_authors: Vec<Author> = vec![
		Author::new(format!("Aizon"), format!("mail@mail.com")),
		Author::new(format!("The"), format!("mail@mail.com")),
		Author::new(format!("Dreaded"), format!("mail@mail.com")),
		Author::new(format!("Hippy"), format!("mail@mail.com")),
	];

	let response = CustomResponse::<Author>::new(
		true,
		Some(format!("Author Created")),
		Some(CustomResponseData::Collection(random_authors))
	);

	Ok(Json(response))
}

pub async fn handler_author_get_specific(State(app_state): State<AppState>, Path(id): Path<i64>) -> Result<Json<CustomResponse<Author>>> {

	let pool = app_state.pool;


	let author: Author = Author { id, name: format!("Resulting Author"), email: format!("result@mail.com") };

	let response = CustomResponse::<Author>::new(
		true,
		Some(format!("Author Created")),
		Some(CustomResponseData::Item(author))
	);

	Ok(Json(response))
}