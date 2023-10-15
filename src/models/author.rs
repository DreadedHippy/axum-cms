use serde::{Deserialize, Serialize};
use sqlx::FromRow;


#[derive(Deserialize, Serialize, Debug, FromRow)]
/// Complete Author model, as-is in the database
pub struct Author {
	pub id: i64,
	pub name: String,
	pub email: String,
	pub password: String
}


#[derive(Deserialize)]
/// Struct holding fields required from client to create an author in the database
pub struct AuthorForCreate {
	pub name: String,
	pub email: String,
	pub password: String
}

#[derive(Deserialize)]
/// Struct holding fields required from client to edit an author
pub struct AuthorForEdit {
	pub name: Option<String>,
}

#[derive(Serialize, Debug)]
/// Struct holding fields to be sent to the client as a resulting Author
pub struct AuthorForResult {
	pub name: String,
	pub email: String
}

impl Author {
	/// Create a dummy author, usually for testing and not much else
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