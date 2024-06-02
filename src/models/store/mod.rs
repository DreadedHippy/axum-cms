// region:    --- Modules

mod error;

pub use self::error::{StoreError, StoreResult};
use crate::config;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

// endregion: --- Modules

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> StoreResult<Db> {
	PgPoolOptions::new()
		.max_connections(5)
		.connect(&config().DB_URL)
		.await
		.map_err(|ex| StoreError::FailToCreatePool(ex.to_string()))
}
