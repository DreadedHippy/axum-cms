use sqlx::FromRow;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, FromRow)]
/// Complete Post model, as-is in the database
pub struct Post {
	pub id: i64,
	pub title: String,
	pub content: String,
	pub author_id: i64
}

#[derive(Deserialize, Debug)]
/// Struct holding fields required from client to create a post in the database
pub struct PostForCreate {
	pub title: String,
	pub content: String
}

#[derive(Deserialize, Debug)]
/// Struct holding fields required from client to edit a post
pub struct PostForEdit {
	pub title: Option<String>,
	pub content: Option<String>
}
#[derive(Debug, Deserialize)]
/// Struct holding request parameters accepted by `post/:id` route
pub struct PostParams {
	pub author: Option<String> // The author's email
}


impl Post {
	/// Create a new post functionally
	pub fn new(title: String, content: String, author_id: i64) -> Self {
		Self { id: 0, title, content, author_id }
	}
}


