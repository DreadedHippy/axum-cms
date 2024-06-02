//! Model Layer
//!
//! Design:
//!
//! - The Model layer normalizes the application's data type
//!   structures and access.
//! - All application code data access must go through the Model layer.
//! - The `ModelManager` holds the internal states/resources
//!   needed by ModelControllers to access data.
//!   (e.g., db_pool, S3 client, redis client).
//! - Model Controllers (e.g., `TaskBmc`, `ProjectBmc`) implement
//!   CRUD and other data access methods on a given "entity"
//!   (e.g., `Task`, `Project`).
//!   (`Bmc` is short for Backend Model Controller).
//! - In frameworks like Axum, Tauri, `ModelManager` are typically used as App State.
//! - ModelManager are designed to be passed as an argument
//!   to all Model Controllers functions.
//!

// region:    --- Modules

pub mod error;
pub mod store;
pub mod state;
pub mod author;
pub mod post;
pub mod edit_suggestion;

use state::AppState;

pub use self::error::{ModelError, ModelResult};

use crate::models::store::{new_db_pool, Db};

// endregion: --- Modules

impl AppState {
	/// Constructor
	pub async fn new() -> ModelResult<Self> {
		let pool = new_db_pool().await?;

		Ok(AppState { pool })
	}

	/// Returns the sqlx db pool reference.
	/// (Only for the model layer)
	pub(in crate::models) fn db(&self) -> &Db {
		&self.pool
	}
}