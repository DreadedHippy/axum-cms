use std::env;

use anyhow::Ok;
use anyhow::Result;
use axum::response::Response;
use chrono::Duration;
use sqlx::postgres::PgConnectOptions;
use sqlx::{PgPool, Pool, Postgres};
use tracing::debug;

pub mod auth;
pub mod cache;
pub mod custom_extractor;

const JWT_DURATION_IN_SECONDS: i64 = 60 * 60 * 2;


pub async fn main_response_mapper(res:Response) -> Response {
	debug!(" {:<12} - main_response_mapper", "RES_MAPPER");

	println!();
	res
}

pub async fn connect_to_postgres() -> Result<Pool<Postgres>> {
	let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

	// Return a Potgres database pool
	Ok(pool)
}