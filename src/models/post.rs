use sqlx::FromRow;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, FromRow)]
pub struct Post {
	pub id: i64,
	pub title: String,
	pub content: String,
	pub author_id: i64
}

#[derive(Deserialize, Debug)]
pub struct PostForCreate {
	pub title: String,
	pub content: String
}

#[derive(Deserialize, Debug)]
pub struct PostForEdit {
	pub title: Option<String>,
	pub content: Option<String>
}
#[derive(Debug, Deserialize)]
pub struct PostParams {
	pub author: Option<String> // The author's email
}


impl Post {
	pub fn new(title: String, content: String, author_id: i64) -> Self {
		Self { id: 0, title, content, author_id }
	}
}


