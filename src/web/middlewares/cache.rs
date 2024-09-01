use axum::Json;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::{Response, IntoResponse};
use redis::AsyncCommands;

use crate::ctx::Ctx;
use crate::models::author::{Author, AuthorForResult};
use crate::web::custom_response::{CustomResponse, CustomResponseData};
use crate::models::post::Post;
use crate::utils::cache::create_redis_connection;
use crate::web::error::{ServerResult, ServerError};

/// Middleware to get cached posts
pub async fn mw_get_cached_posts<B>(
	req: Request<B>,
	next: Next<B>
) -> ServerResult<Response> {
	let mut conn = create_redis_connection().await.map_err(|_| {
		ServerError::CouldNotConnectToRedis
	})?;

	let results: std::result::Result<String, ServerError> = conn.get("posts").await.map_err(|_| {
		ServerError::InternalServerError
	});

	match results {
		Ok(posts) => {
			let data: Vec<Post> = serde_json::from_str(&posts).map_err(|_| {
				ServerError::InternalServerError
			})?;

			let response = CustomResponse::<Post>::new(
				true,
				Some(format!("Posts retrieved successfully [CACHE]")),
				Some(CustomResponseData::Collection(data))
			);
			
			Ok(Json(response).into_response())
		},

		Err(_) => {
			Ok(next.run(req).await)
		}
	}

}

/// Middleware to get cached authors
pub async fn mw_get_cached_authors<B>(
	req: Request<B>,
	next: Next<B>
) -> ServerResult<Response> {
	let mut conn = create_redis_connection().await.map_err(|_| {
		ServerError::CouldNotConnectToRedis
	})?;

	let results: std::result::Result<String, ServerError> = conn.get("authors").await.map_err(|_| {
		ServerError::InternalServerError
	});

	match results {
		Ok(authors) => {
			let data: Vec<Author> = serde_json::from_str(&authors).map_err(|_| {
				ServerError::InternalServerError
			})?;

			let data = data.into_iter().map(AuthorForResult::from).collect::<Vec<AuthorForResult>>();

			let response = CustomResponse::<AuthorForResult>::new(
				true,
				Some(format!("Authors retrieved successfully [CACHE]")),
				Some(CustomResponseData::Collection(data))
			);
			
			Ok(Json(response).into_response())
		},

		Err(_) => {
			Ok(next.run(req).await)
		}
	}

}