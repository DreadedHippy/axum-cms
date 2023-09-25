use serde::{Deserialize, Serialize};
use sqlx::FromRow;


#[derive(Serialize, Debug, FromRow)]
pub struct Author {
	pub id: i64,
	pub name: String,
	pub email: String
}


#[derive(Deserialize)]
pub struct AuthorForCreate {
	pub name: String,
	pub email: String
}

impl Author {
	pub fn new(name: String, email: String) -> Self {
		// TODO: Implement actual ID generation or retrieval from DB
		Self {
			id: 0,
			name,
			email
		}
	}
}