use serde::{Deserialize, Serialize};
use sqlx::FromRow;


#[derive(Deserialize, Serialize, Debug, FromRow)]
pub struct Author {
	pub id: i64,
	pub name: String,
	pub email: String,
	pub password: String
}


#[derive(Deserialize)]
pub struct AuthorForCreate {
	pub name: String,
	pub email: String,
	pub password: String
}

#[derive(Deserialize)]
pub struct AuthorForEdit {
	pub name: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct AuthorForResult {
	pub name: String,
	pub email: String
}

impl Author {
	pub fn new_dummy(name: String, email: String, password: String) -> Self {
		// TODO: Implement actual ID generation or retrieval from DB
		Self {
			id: 0,
			name,
			email,
			password
		}
	}
}