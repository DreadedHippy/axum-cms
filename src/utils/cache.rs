use std::{env, error::Error};

// use redis::{Client, RedisError, Connection, Commands};
use redis::{aio::Connection, AsyncCommands, FromRedisValue, Commands, Client};
use anyhow::Result;
use tracing::info;

use crate::models::{post::Post, author::Author};


/// Get a connection to redis
pub async fn create_redis_connection() -> Result<Connection>{
	let (redis_conn_url) = match env::var("MODE") {
		Ok(mode) => {
				if mode == String::from("production") {
						env::var("PROD_REDIS_CONN_URL").expect("Env variable `PROD_REDIS_CONN_URL` not found")
				} else {
						env::var("DEV_REDIS_CONN_URL").expect("Env variable `DEV_REDIS_CONN_URL` not found")
				}
		},
		_ => {
			env::var("DEV_REDIS_CONN_URL").expect("Env variable `DEV_REDIS_CONN_URL` not found")
		}
	};

	let client = Client::open(redis_conn_url)?;
	let mut conn = client.get_tokio_connection().await?;

	Ok(conn)
}

/// Update posts in the Redis Cache
pub async fn  update_cached_posts(posts: &Vec<Post>) -> Result<()>{
	let data = serde_json::to_string(posts).expect("Could not serialize posts");
	let mut conn = create_redis_connection().await.expect("Could not create redis connection");
	let _: () = conn.set("posts", data).await.expect("Could not update cached posts");

	Ok(())
}

/// Update authors in the redis cache
pub async fn  update_cached_authors(authors: &Vec<Author>) -> Result<()>{
	let data = serde_json::to_string(&authors).expect("Could not serialize authors");
	let mut conn = create_redis_connection().await.expect("Could not create redis connection");
	let _: () = conn.set("authors", data).await.expect("Could not update cached authors");

	Ok(())
}

/// Initialize the cache, ideally to be used on server startup
pub async fn initialize_cache(authors: Vec<Author>, posts: Vec<Post>) -> Result<()> {
	//TODO: Use different cache url depending on dev mode or prod mode
	let posts_data = serde_json::to_string(&posts).expect("Could not serialize posts");
	let authors_data = serde_json::to_string(&authors).expect("Could not serialize authors");

	let mut conn = create_redis_connection().await.expect("Could not create redis connection");
	
	let _: () = conn.set("posts", posts_data).await.expect("Could not initialize posts cache");
	let _: () = conn.set("authors", authors_data).await.expect("Could not initialize authors cache");

	info!("INITIALIZED CACHE SUCCESSFULLY");

	// region: Deserialization tests

	// let authors_result: String = conn.get("authors").await?;
	// let posts_result: String = conn.get("posts").await?;

	// let authors: Vec<Author> = serde_json::from_str(&authors_result).unwrap();
	// let posts: Vec<Post> = serde_json::from_str(&posts_result).unwrap();

	// println!("THE AUTHORS");
	// println!("{:#?}", authors);

	// println!("THE POSTS");
	// println!("{:#?}", posts);



	// debug!(" THE END");

	// endregion: Deserialization tests


	Ok(())
}




