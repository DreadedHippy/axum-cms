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

/// A response mapper for the entire app :shrug:
pub async fn main_response_mapper(res:Response) -> Response {
	debug!(" {:<12} - main_response_mapper", "RES_MAPPER");

	println!();
	res
}

/// Create a postgres db pool , create necessary tables, return the connection
pub async fn connect_to_postgres(database_url: String) -> Result<Pool<Postgres>> {
	let pool = PgPool::connect(&database_url).await?;

	// Create authors table if absent
	let create_authors_table_query = r#"
		CREATE TABLE IF NOT EXISTS authors (
			id BIGINT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
			name varchar(128) NOT NULL,
			email varchar(128) NOT NULL UNIQUE,
			password varchar(256) NOT NULL
		)
	"#;

	let _ = sqlx::query(create_authors_table_query).execute(&pool).await?;

	// Create posts table if absent
	let create_posts_table_query = r#"
		CREATE TABLE IF NOT EXISTS posts (
			id BIGINT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
			title varchar(256) NOT NULL,
			content varchar(512) NOT NULL,
			author_id BIGINT,
			FOREIGN KEY (author_id) REFERENCES authors(id) ON DELETE SET NULL
		)
	"#;

	let _ = sqlx::query(create_posts_table_query).execute(&pool).await?;
	// Create EDIT_STATUS enum
	let create_edit_status_enum_query = r#"
	DO $$ BEGIN
		CREATE TYPE EDIT_STATUS AS ENUM ('PENDING', 'ACCEPTED', 'REJECTED');
	EXCEPTION
		WHEN duplicate_object THEN null;
	END $$;
	"#;

	let _ = sqlx::query(create_edit_status_enum_query).execute(&pool).await?;

	// Create posts table if absent
	let create_edits_table_query = r#"
		CREATE TABLE IF NOT EXISTS edits (
			post_id BIGINT NOT NULL,
			author_id BIGINT NOT NULL,
			status EDIT_STATUS NOT NULL DEFAULT 'PENDING'::EDIT_STATUS,
			new_content text NOT NULL,
			FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE SET NULL,
			FOREIGN KEY (author_id) REFERENCES authors(id) ON DELETE SET NULL,
			PRIMARY KEY (author_id, post_id)
		)
	"#;

	let _ = sqlx::query(create_edits_table_query).execute(&pool).await?;

	// Return a Postgres database pool
	Ok(pool)
}