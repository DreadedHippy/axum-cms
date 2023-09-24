use serde::{Deserialize, Serialize};


#[derive(Serialize)]
pub struct Post {
	pub id: u64,
	pub title: String,
	pub content: String,
	pub author_id: u64
}

#[derive(Deserialize)]
pub struct PostForCreate {
	pub title: String,
	pub content: String,
	pub author_id: u64
}

impl Post {
	pub fn new(title: String, content: String, author_id: u64) -> Self {
		Self { id: 0, title, content, author_id }
	}
}
