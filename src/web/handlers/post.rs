use axum::{extract::{Path, Query, State}, http::StatusCode, Extension, Json};
use axum_extra::extract::WithRejection;
use tracing::debug;

use crate::{ctx::Ctx, models::{post::{self, Post, PostBmc, PostForCreate, PostForUpdate}, AppState}, web::{error::CrudError, IncomingServerRequest, ServerResponse}};
use crate::web::custom_extractor::ApiError;
use crate::web::{error::{ServerResult, ServerError}, custom_response::{CustomResponse, CustomResponseData}};

/// Handler to create a post
pub async fn handler_post_create(
	State(app_state): State<AppState>,
	ctx: Ctx,
	WithRejection((Json(data)), _): IncomingServerRequest<PostForCreate>,
	) -> ServerResponse<Post> {
	debug!("{:<12} - handler_post_create", "HANDLER");
	
	let author_id = ctx.user_id();

	let id = PostBmc::create(&ctx, &app_state, data).await?;

	let post = PostBmc::get(&ctx, &app_state, id).await?;

	let response = CustomResponse::new(
		true,
		Some(format!("Post created successfully")),
		Some(CustomResponseData::Item(post))
	);

	Ok((StatusCode::CREATED, Json(response)))
}

/// Handler to get every post from every author all at once
pub async fn handler_post_list(
	State(app_state): State<AppState>
) -> ServerResponse<Post> {
	debug!("{:<12} - handler_post_list", "HANDLER");
	let posts = PostBmc::list(&app_state, None, None).await?;

	let response = CustomResponse::new(
		true,
		Some(format!("Posts retrieved successfully")),
		Some(CustomResponseData::Collection(posts))
	);

	Ok((StatusCode::OK,  Json(response)))
}

// Handler to get a specific post
pub async fn handler_post_get(
	ctx: Ctx,
	State(app_state): State<AppState>,
	Path(id): Path<i64>
) -> ServerResponse<Post> {

	let post: Post = PostBmc::get(&ctx, &app_state, id).await?;

	let response  = CustomResponse::new(
		true,
		Some(format!("Post retrieved successfully")),
		Some(CustomResponseData::Item(post))
	);

	Ok((StatusCode::OK, Json(response)))
}

/// Handler to update a specific post
pub async fn handler_post_update(
	ctx: Ctx,
	State(app_state): State<AppState>,
	Path(id): Path<i64>,
	WithRejection((Json(post_e)), _): IncomingServerRequest<PostForUpdate>
	) -> ServerResponse<Post> {
	debug!("{:<12} - handler_post_update", "HANDLER");
	let post = PostBmc::get(&ctx, &app_state, id).await?;

	let author_id = ctx.user_id();

	if post.author_id != author_id {
		return Err(
			ServerError::UpdateFail(
				"Post".to_string(),
				"Only post author can update post".to_string(),
				CrudError::UNAUTHORIZED
			)
		)
	}

	let _result = PostBmc::update(&ctx, &app_state, id, post_e).await?;

	let post = PostBmc::get(&ctx, &app_state, id).await?;

	let response  = CustomResponse::new(
		true,
		Some(format!("Post updated successfully")),
		Some(CustomResponseData::Item(post))
	);

	Ok((StatusCode::ACCEPTED, Json(response)))
}

/// Handler to delete a post
pub async fn handler_post_delete(
	ctx: Ctx,
	State(app_state): State<AppState>,
	Path(id): Path<i64>,
	) -> ServerResponse<Post> {

	debug!("{:<12} - handler_post_delete", "HANDLER");

	let post = PostBmc::get(&ctx, &app_state, id).await?;

	let author_id = ctx.user_id();

	if post.author_id != author_id {
		return Err(
			ServerError::UpdateFail(
				"Post".to_string(),
				"Only post author can delete post".to_string(),
				CrudError::UNAUTHORIZED
			)
		)
	}

	let _result = PostBmc::delete(&ctx, &app_state, id).await?;

	let response  = CustomResponse::new(
		true,
		Some(format!("Post deleted successfully")),
		None
	);

	Ok((StatusCode::OK, Json(response)))
}