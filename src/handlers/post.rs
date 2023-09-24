use axum::{Json, extract::Path};

use crate::models::{error::{Result, Error}, custom_response::{CustomResponse, CustomResponseData}, post::{Post, PostForCreate, self}};

pub async fn handler_post_create(Json(post_info): Json<PostForCreate>) -> Result<Json<CustomResponse<Post>>>{
	let post = Post::new(post_info.title, post_info.content, post_info.author_id);

	let response  = CustomResponse::new(
		true,
		Some(format!("Posts retrieved successfully")),
		Some(CustomResponseData::Item(post))
	);

	Ok(Json(response))
}

pub async fn handler_post_get_all() -> Result<Json<CustomResponse<Post>>>{
	let random_posts = vec![
		Post::new(format!("Post 1"), format!("These are the contents of post 1"), 0),
		Post::new(format!("Post 2"), format!("These are the contents of post 2"), 0),
		Post::new(format!("Post 3"), format!("These are the contents of post 3"), 0),
		Post::new(format!("Post 4"), format!("These are the contents of post 4"), 0),
	];

	let response  = CustomResponse::new(
		true,
		Some(format!("Posts retrieved successfully")),
		Some(CustomResponseData::Collection(random_posts))
	);

	Ok(Json(response))
}

pub async fn handler_post_get_specific(Path(id): Path<u64>) -> Result<Json<CustomResponse<Post>>>{

	let retrieved_post: Post = Post {
		id,
		title: format!("Dummy post title"),
		content: format!("Dummy post content"),
		author_id: 1
	};

	let response  = CustomResponse::new(
		true,
		Some(format!("Post retrieved successfully")),
		Some(CustomResponseData::Item(retrieved_post))
	);

	Ok(Json(response))
}