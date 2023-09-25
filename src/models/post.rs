use sqlx::FromRow;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Debug, FromRow)]
pub struct Post {
	pub id: i64,
	pub title: String,
	pub content: String,
	pub author_id: i64
}

#[derive(Deserialize, Debug)]
pub struct PostForCreate {
	pub title: String,
	pub content: String,
	pub author_id: i64
}

impl Post {
	pub fn new(title: String, content: String, author_id: i64) -> Self {
		Self { id: 0, title, content, author_id }
	}
}
