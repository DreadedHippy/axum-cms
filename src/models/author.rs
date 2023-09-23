use serde::{Deserialize, Serialize};


#[derive(Serialize)]
pub struct Author {
	pub id: u64,
	pub name: String
}


#[derive(Deserialize)]
pub struct AuthorForCreate {
	pub name: String
}

impl Author {
	pub fn new(name: String) -> Self {
		// TODO: Implement actual ID generation or retrieval from DB
		Self {
			id: 0,
			name
		}
	}
}