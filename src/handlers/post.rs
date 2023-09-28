use axum::{Json, extract::{Path, State, Query}};

use crate::models::{error::{Result, Error}, custom_response::{CustomResponse, CustomResponseData}, post::{Post, PostForCreate, self, PostParams}, state::AppState};

pub async fn handler_post_create(State(app_state): State<AppState>, Json(post_info): Json<PostForCreate>) -> Result<Json<CustomResponse<Post>>>{
	let result = app_state.create_post(post_info).await.map_err(|e| {
		Error::CouldNotCreatePost
	})?;

	let response  = CustomResponse::new(
		true,
		Some(format!("Post created successfully")),
		Some(CustomResponseData::Item(result))
	);

	Ok(Json(response))
}

pub async fn handler_post_get_all(Query(params): Query<PostParams>, State(app_state): State<AppState>) -> Result<Json<CustomResponse<Post>>>{
	let posts = match params.author {
		Some(email) => {
			app_state.get_posts_by_author(email).await.map_err(|e| {
				Error::CouldNotGetPosts
			})?
			
		}

		None => {
			app_state.get_all_posts().await.map_err(|e| {
				Error::CouldNotGetPosts
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

pub async fn handler_post_get_specific(State(app_state): State<AppState>,  Path(id): Path<i64>) -> Result<Json<CustomResponse<Post>>>{

	let retrieved_post: Post = app_state.get_specific_post(id).await.map_err(|e| {
		Error::CouldNotGetPost
	})?;

	let response  = CustomResponse::new(
		true,
		Some(format!("Post retrieved successfully")),
		Some(CustomResponseData::Item(retrieved_post))
	);

	Ok(Json(response))
}


// pub async fn handler_get_posts_by_author(Query(params): Query<PostParams>, State(app_state): State<AppState>) -> Result<Json<CustomResponse<Post>>>{
// 	match params.author {
// 		Some(email) => {
// 			let author_posts: Vec<Post> = app_state.get_posts_by_author(email).await.map_err(|_| {
// 				Error::CouldNotGetPosts
// 			})?;		
		
// 			let response  = CustomResponse::new(
// 				true,
// 				Some(format!("Post retrieved successfully")),
// 				Some(CustomResponseData::Collection(author_posts))
// 			);
		
// 			Ok(Json(response))

// 		},

// 		None => {
// 			let retrieved_post: Post = app_state.get_specific_post(id).await.map_err(|e| {
// 				Error::CouldNotGetPost
// 			})?;
		
// 			let response  = CustomResponse::new(
// 				true,
// 				Some(format!("Post retrieved successfully")),
// 				Some(CustomResponseData::Item(retrieved_post))
// 			);
		
// 			Ok(Json(response))
// 		}
// 	}
// }