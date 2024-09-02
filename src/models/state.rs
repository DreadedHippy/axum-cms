use std::thread;

use sqlx::{Pool, Postgres, Error, Row};
use tokio::runtime::Runtime;
use tracing::debug;
use crate::{models::author::{Author, AuthorForResult}, web::handlers::auth, ServerResult};
use super::{author::AuthorForCreate, edit::{Edit, EditForCreate}, post::{Post, PostForCreate}};
use super::store::{new_db_pool, Db};



#[derive(Clone)]
/// Struct holding the application state
pub struct AppState {
	pub pool: Db
}


// impl AppState {
// 	// #[tokio::main]
// 	/// Update the cache of authors
// 	pub async fn update_authors_cache(&self) {
// 		if let Ok(authors) = self.get_all_authors().await {

// 			update_cached_authors(&authors).await.expect("Failed to update cached authors");
// 			debug!(" {:<12 } - Cached Authors updated", "CACHE");

// 		} else {
// 			debug!("{:<12} Cache update failed", "AUTHORS")
// 		}
// 	}
// }

