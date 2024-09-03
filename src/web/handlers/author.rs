use std::fmt::format;

use axum::http::StatusCode;
use axum::{debug_handler, Extension, Json};
use axum::extract::{Path, State};
use axum_extra::extract::WithRejection;
use tracing::{debug, info};

use crate::ctx::Ctx;
use crate::models::author::{Author, AuthorBmc, AuthorForCreate, AuthorForEdit};
use crate::web::custom_response::{CustomResponse, CustomResponseData};
use crate::web::error::{ServerResult, ServerError};
use crate::web::custom_extractor::ApiError;
use crate::models::AppState;
use crate::web::{IncomingServerRequest, ServerResponse};

pub async fn handler_author_create(
	State(app_state): State<AppState>,
	WithRejection(Json(data), _): IncomingServerRequest<AuthorForCreate>
) -> ServerResponse<Author> {
	debug!(" {:<12} - handler_author_create", "HANDLER");

	let id = AuthorBmc::create_no_auth(&app_state, data).await?;

	let author: Author = AuthorBmc::get_no_auth(&app_state, id).await?;

	let response = CustomResponse::<Author>::new(
		true,
		Some(format!("Author Created")),
		Some(CustomResponseData::Item(author))
	);

	Ok((StatusCode::CREATED, Json(response)))
}

/// Handler to list all authors
pub async fn handler_author_list(State(app_state): State<AppState>) -> ServerResponse<Author> {
	debug!("{:>12} - handler_author", "HANDLER");

	let authors = AuthorBmc::list(&app_state, None, None).await?;

	let response = CustomResponse::<Author>::new(
		true,
		Some(format!("Authors Retrieved")),
		Some(CustomResponseData::Collection(authors))
	);

	Ok((StatusCode::OK, Json(response)))
}

/// Handler to get an author
pub async fn handler_author_get(State(app_state): State<AppState>, Path(id): Path<i64>) -> ServerResponse<Author> {
	debug!("{:>12} - handler_author", "HANDLER");

	let author: Author = AuthorBmc::get_no_auth(&app_state, id).await?;

	let response = CustomResponse::new(
		true,
		Some(format!("Author Retrieved")),
		Some(CustomResponseData::Item(author))
	);

	Ok((StatusCode::OK, Json(response)))
}