use std::env;

use anyhow::Ok;
use anyhow::Result;
use axum::response::Response;
use sqlx::postgres::PgConnectOptions;
use sqlx::{PgPool, Pool, Postgres};


pub async fn main_response_mapper(res:Response) -> Response {
	println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

	println!();
	res
}

pub async fn connect_to_postgres() -> Result<Pool<Postgres>> {
	let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

	// let pool: PgPool = Pool::<Postgres>::connect_with(pool_options).await?;
	Ok(pool)
}