use axum::{Json, extract::{Path, State, Query}, Extension};
use axum_extra::extract::WithRejection;

use crate::{models::{error::{ServerResult, ServerError}, custom_response::{CustomResponse, CustomResponseData}, post::{Post, PostForCreate, self, PostParams, PostForEdit}, state::AppState}, utils::{custom_extractor::ApiError, auth::get_info_from_jwt}};

/// Handler to create a post
pub async fn handler_post_create(
	State(app_state): State<AppState>,
	Extension(token): Extension<String>,
	WithRejection((Json(post_info)), _): WithRejection<Json<PostForCreate>, ApiError>,
	) -> ServerResult<Json<CustomResponse<Post>>>{
	let (_, author_id) = get_info_from_jwt(token)?;

	let result = app_state.create_post(post_info, author_id).await.map_err(|e| {
		ServerError::CouldNotCreatePost
	})?;

	let response  = CustomResponse::new(
		true,
		Some(format!("Post created successfully")),
		Some(CustomResponseData::Item(result))
	);

	Ok(Json(response))
}

/// Handler to get every post from every author all at once
pub async fn handler_post_get_all(Query(params): Query<PostParams>, State(app_state): State<AppState>) -> ServerResult<Json<CustomResponse<Post>>>{
	let posts = match params.author {
		Some(email) => {
			app_state.get_posts_by_author(email).await.map_err(|e| {
				ServerError::CouldNotGetPosts
			})?
			
		}

		None => {
			app_state.get_all_posts().await.map_err(|e| {
				ServerError::CouldNotGetPosts
			})?
		}
	};
	
	

	let response  = CustomResponse::new(
		true,
		Some(format!("Posts retrieved successfully")),
		Some(CustomResponseData::Collection(posts))
	);

	Ok(Json(response))
}

// Handler to get a specific post
pub async fn handler_post_get_specific(State(app_state): State<AppState>,  Path(id): Path<i64>) -> ServerResult<Json<CustomResponse<Post>>>{

	let retrieved_post: Post = app_state.get_specific_post(id).await.map_err(|e| {
		ServerError::CouldNotGetPost
	})?;

	let response  = CustomResponse::new(
		true,
		Some(format!("Post retrieved successfully")),
		Some(CustomResponseData::Item(retrieved_post))
	);

	Ok(Json(response))
}

/// Handler to edit a specific post
pub async fn handler_post_edit(
	Extension(token): Extension<String>,
	State(app_state): State<AppState>,
	Path(post_id): Path<i64>,
	WithRejection((Json(post)), _): WithRejection<Json<PostForEdit>, ApiError>
	) -> ServerResult<Json<CustomResponse<Post>>>{

	let title = post.title.unwrap_or_default();
	let content = post.content.unwrap_or_default();
	let (editor_email, editor_id) = get_info_from_jwt(token)?; // Get email of editor from token

	// Get original post author from token
	let original_author_id = app_state.get_post_author_id(post_id).await.map_err(|e| {
		ServerError::CouldNotEditPost
	})?;

	// If editor is not original post author, reject
	if original_author_id != editor_id {
		return Err(ServerError::OnlyAuthorCanEdit)
	}

	// Edit post if all conditions pass
	let edited_post: Post = app_state.edit_post(title, content, post_id).await.map_err(|e| {
		ServerError::CouldNotEditPost
	})?;

	let response  = CustomResponse::new(
		true,
		Some(format!("Post updated successfully")),
		Some(CustomResponseData::Item(edited_post))
	);

	Ok(Json(response))
}

/// Handler to delete a post
pub async fn handler_post_delete(State(app_state): State<AppState>,  Path(id): Path<i64>) -> ServerResult<Json<CustomResponse<Post>>>{
	// Delete the post, we don't care about the result, it only should throw no error
	let _ = app_state.delete_post(id).await.map_err(|e| {
		ServerError::CouldNotDeletePost
	})?;

	let response  = CustomResponse::new(
		true,
		Some(format!("Post deleted successfully")),
		None
	);

	Ok(Json(response))
}