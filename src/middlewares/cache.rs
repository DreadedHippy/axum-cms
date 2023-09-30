use axum::Json;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::{Response, IntoResponse};
use redis::AsyncCommands;

use crate::models::author::Author;
use crate::models::custom_response::{CustomResponse, CustomResponseData};
use crate::models::post::Post;
use crate::utils::cache::create_redis_connection;
use crate::models::error::{Result, Error};

pub async fn mw_get_cached_posts<B>(
	req: Request<B>,
	next: Next<B>
) -> Result<Response> {
	let mut conn = create_redis_connection().await.map_err(|_| {
		Error::CouldNotConnectToRedis
	})?;

	let results: std::result::Result<String, Error> = conn.get("posts").await.map_err(|_| {
		Error::InternalServerError
	});

	match results {
		Ok(posts) => {
			let data: Vec<Post> = serde_json::from_str(&posts).map_err(|_| {
				Error::InternalServerError
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


pub async fn mw_get_cached_authors<B>(
	req: Request<B>,
	next: Next<B>
) -> Result<Response> {
	let mut conn = create_redis_connection().await.map_err(|_| {
		Error::CouldNotConnectToRedis
	})?;

	let results: std::result::Result<String, Error> = conn.get("authors").await.map_err(|_| {
		Error::InternalServerError
	});

	match results {
		Ok(authors) => {
			let data: Vec<Author> = serde_json::from_str(&authors).map_err(|_| {
				Error::InternalServerError
			})?;

			let response = CustomResponse::<Author>::new(
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