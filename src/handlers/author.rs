use axum::Json;

use crate::models::author::{Author, AuthorForCreate};
use crate::models::custom_response::{CustomResponse, CustomResponseData};
use crate::models::error::Result;

pub async fn handler_author_create(Json(author): Json<AuthorForCreate>) -> Result<Json<CustomResponse<Author>>> {
	let author_result = Author::new(author.name);

	let response = CustomResponse::<Author>::new(
		true,
		Some(format!("Author Created")),
		Some(CustomResponseData::Item(author_result))
	);

	Ok(Json(response))
}

pub async fn handler_author_get_all() -> Result<Json<CustomResponse<Author>>> {

	let random_authors: Vec<Author> = vec![
		Author::new(format!("Aizon")),
		Author::new(format!("The")),
		Author::new(format!("Dreaded")),
		Author::new(format!("Hippy")),
	];

	let response = CustomResponse::<Author>::new(
		true,
		Some(format!("Author Created")),
		Some(CustomResponseData::Collection(random_authors))
	);

	Ok(Json(response))
}
