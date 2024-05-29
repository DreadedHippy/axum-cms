use std::thread;

use sqlx::{Pool, Postgres, Error, Row};
use tokio::runtime::Runtime;
use tracing::debug;
use crate::{models::author::{Author, AuthorForResult}, utils::cache::{update_cached_posts, update_cached_authors}, handlers::auth};
use super::{author::AuthorForCreate, edit_suggestion::{EditSuggestion, EditSuggestionForCreate}, post::{Post, PostForCreate}};

#[derive(Clone)]
/// Struct holding the application state
pub struct AppState {
	pub pool: Pool<Postgres>
}

impl AppState {
	// region: --Database Manipulations for authors
	/// Create an author in the database via SQLX
	pub async fn create_author(&self, author_info: AuthorForCreate) -> Result<Author, Error>{
		let q = r#"
		INSERT INTO authors (name, email, password)
		VALUES( $1, $2, $3)
		RETURNING *
		"#;

		let result = sqlx::query_as::<_, Author>(q)
		.bind(author_info.name)
		.bind(author_info.email)
		.bind(author_info.password)
		.fetch_one(&self.pool)
		.await?;

		// println!("{:#?}", result);

    Ok(result)
		
	}

	/// Get all authors from the database
	pub async fn get_all_authors(&self) -> Result<Vec<Author>, Error> {
		let q = r#"
		SELECT * FROM authors
		"#;

		let records = sqlx::query_as::<_, Author>(q);

		let authors = records
		.fetch_all(&self.pool)
		.await?;

	

		Ok(authors)
	}	

	/// Get specific author from the database
	pub async fn get_specific_author(&self, id: i64) -> Result<Author, Error> {
		let q = r#"
		SELECT * FROM authors where id = $1
		"#;

		let record = sqlx::query_as::<_, Author>(q);

		let author = record
		.bind(id)
		.fetch_one(&self.pool)
		.await?;

		Ok(author)
	}

	/// Edit an author in the database, returning the updated author
	pub async fn edit_author(&self, name: String, id: i64) -> Result<Author, Error> {
		let q = r#"
		UPDATE authors
		SET name = COALESCE(
			NULLIF($1, ''),
			name
		)
		WHERE id = $2
		RETURNING *
		"#;

		let record = sqlx::query_as::<_, Author>(q);

		let author = record
		.bind(name)
		.bind(id)
		.fetch_one(&self.pool)
		.await?;

		// Update cache
		self.update_authors_cache().await;

		Ok(author)
	}

	/// Delete an author from the database
	pub async fn delete_author(&self, id: i64) -> Result<bool, Error> {
		let q = r#"
		DELETE FROM authors
		WHERE id = $1
		"#;

		let record = sqlx::query(q);

		let post = record
		.bind(id)
		.execute(&self.pool)
		.await?;

	
		// Update cache
		self.update_authors_cache().await;

		return Ok(true);
	}

	/// Get an author from the database given their email
	pub async fn get_author_by_email(&self, email: String) -> Result<Author, Error> {
		let q = r#"
		SELECT * FROM authors where email = $1
		"#;

		let record = sqlx::query_as::<_, Author>(q);

		let author = record
		.bind(email)
		.fetch_one(&self.pool)
		.await?;

		Ok(author)
	}
	// endregion: --Database Manipulations for authors
}

impl AppState {
	// region: --Database Manipulations for posts
	/// Create a post in the database via SQLX
	pub async fn create_post(&self, post_info: PostForCreate, author_id: i64) -> Result<Post, Error>{
		let q = r#"
		INSERT INTO posts (title, content, author_id)
		VALUES( $1, $2, $3)
		RETURNING *
		"#;

		let rec = sqlx::query(q)
		.bind(post_info.title)
		.bind(post_info.content)
		.bind(author_id)
		.fetch_one(&self.pool)
		.await?;

		let result = Post {
			id: rec.get("id"),
			title: rec.get("title"),
			content: rec.get("content"),
			author_id: rec.get("author_id")
		};

		// println!("{:#?}", result);

		// Update cache
		self.update_posts_cache().await;

    Ok(result)		
	}

	/// Get all posts from the database
	pub async fn get_all_posts(&self) -> Result<Vec<Post>, Error> {
		let q = r#"
		SELECT * FROM posts
		"#;

		let records = sqlx::query_as::<_, Post>(q);

		let posts = records
		.fetch_all(&self.pool)
		.await?;
	
		Ok(posts)
	}

	/// Get specific post from the database
	pub async fn get_specific_post(&self, id: i64) -> Result<Post, Error> {
		let q = r#"
		SELECT * FROM posts where id = $1
		"#;

		let record = sqlx::query_as::<_, Post>(q);

		let post = record
		.bind(id)
		.fetch_one(&self.pool)
		.await?;

		Ok(post)
	}

	/// Get the author id of a specific post given the id of the post
	pub async fn get_post_author_id(&self, post_id: i64) -> Result<i64, Error>{
		let q = r#"
			SELECT author_id
			FROM posts
			WHERE id = $1
		"#;

		let row: (i64, ) = sqlx::query_as(q).bind(post_id)
			.fetch_one(&self.pool)
			.await?;
		Ok(row.0)
	}

	/// Edit a post in the database, returning the updated post
	pub async fn edit_post(&self, title: String, content: String, id: i64) -> Result<Post, Error> {

		let q = r#"
		UPDATE posts
		SET title = COALESCE(
			NULLIF($1, ''),
			title
		),
		content = COALESCE(
			NULLIF($2, ''),
			content
		)
		WHERE id = $3
		RETURNING *
		"#;

		let record = sqlx::query_as::<_, Post>(q);

		let post = record
		.bind(title)
		.bind(content)
		.bind(id)
		.fetch_one(&self.pool)
		.await?;

	
		// Update cache using "GET ALL functionality"		
		self.update_posts_cache().await;

		return Ok(post);
	}

	
	/// Delete a post post from the database
	pub async fn delete_post(&self, id: i64) -> Result<bool, Error> {
		let q = r#"
		DELETE FROM posts
		WHERE id = $1
		"#;

		let record = sqlx::query(q);

		let post = record
		.bind(id)
		.execute(&self.pool)
		.await?;

	
		// Update cache using "GET ALL functionality"		
		self.update_posts_cache().await;

		return Ok(true);
	}

	/// Get all posts by a specific author, given the author's email
	pub async fn get_posts_by_author(&self, email: String) -> Result<Vec<Post>, Error> {
		// println!("{}", email);
		let q = r#"
		SELECT *
		FROM posts p
		WHERE p.author_id IN (
			SELECT id
			FROM authors
			WHERE email = $1
		)
		"#;

		let record = sqlx::query_as::<_, Post>(q);

		let post = record
		.bind(email)
		.fetch_all(&self.pool)
		.await?;

		Ok(post)
	}

	// endregion: --Database Manipulations for posts
}


impl AppState {
	// region: --Database Manipulations for edit suggestions
	/// Create a post in the database via SQLX
	pub async fn create_edit_suggestion(&self, edit_suggestion_info: EditSuggestionForCreate, author_id: i64) -> Result<EditSuggestion, Error>{
		let q = r#"
		INSERT INTO edit_suggestion (author_id, post_id, new_content)
		VALUES( $1, $2, $3)
		RETURNING *
		"#;

		let rec = sqlx::query(q)
		.bind(author_id)
		.bind(edit_suggestion_info.post_id)
		.bind(edit_suggestion_info.new_content)
		.fetch_one(&self.pool)
		.await?;

		let result = EditSuggestion {
			post_id: rec.get("post_id"),
			author_id: rec.get("author_id"),
			new_content: rec.get("new_content"),
			status: rec.get("status")
		};

		// Todo: Update cache
		// let posts = self.get_all_posts().await?;

    Ok(result)		
	}

	// /// Get all posts from the database
	// pub async fn get_all_posts(&self) -> Result<Vec<Post>, Error> {
	// 	let q = r#"
	// 	SELECT * FROM posts
	// 	"#;

	// 	let records = sqlx::query_as::<_, Post>(q);

	// 	let posts = records
	// 	.fetch_all(&self.pool)
	// 	.await?;
	
	// 	Ok(posts)
	// }

	// /// Get specific post from the database
	// pub async fn get_specific_post(&self, id: i64) -> Result<Post, Error> {
	// 	let q = r#"
	// 	SELECT * FROM posts where id = $1
	// 	"#;

	// 	let record = sqlx::query_as::<_, Post>(q);

	// 	let post = record
	// 	.bind(id)
	// 	.fetch_one(&self.pool)
	// 	.await?;

	// 	Ok(post)
	// }

	// /// Get the author id of a specific post given the id of the post
	// pub async fn get_post_author_id(&self, post_id: i64) -> Result<i64, Error>{
	// 	let q = r#"
	// 		SELECT author_id
	// 		FROM posts
	// 		WHERE id = $1
	// 	"#;

	// 	let row: (i64, ) = sqlx::query_as(q).bind(post_id)
	// 		.fetch_one(&self.pool)
	// 		.await?;
	// 	Ok(row.0)
	// }

	// /// Edit a post in the database, returning the updated post
	// pub async fn edit_post(&self, title: String, content: String, id: i64) -> Result<Post, Error> {

	// 	let q = r#"
	// 	UPDATE posts
	// 	SET title = COALESCE(
	// 		NULLIF($1, ''),
	// 		title
	// 	),
	// 	content = COALESCE(
	// 		NULLIF($2, ''),
	// 		content
	// 	)
	// 	WHERE id = $3
	// 	RETURNING *
	// 	"#;

	// 	let record = sqlx::query_as::<_, Post>(q);

	// 	let post = record
	// 	.bind(title)
	// 	.bind(content)
	// 	.bind(id)
	// 	.fetch_one(&self.pool)
	// 	.await?;

	
	// 	// Update cache using "GET ALL functionality"		
	// 	self.update_posts_cache().await;

	// 	return Ok(post);
	// }

	
	// /// Delete a post post from the database
	// pub async fn delete_post(&self, id: i64) -> Result<bool, Error> {
	// 	let q = r#"
	// 	DELETE FROM posts
	// 	WHERE id = $1
	// 	"#;

	// 	let record = sqlx::query(q);

	// 	let post = record
	// 	.bind(id)
	// 	.execute(&self.pool)
	// 	.await?;

	
	// 	// Update cache using "GET ALL functionality"		
	// 	self.update_posts_cache().await;

	// 	return Ok(true);
	// }

	// /// Get all posts by a specific author, given the author's email
	// pub async fn get_posts_by_author(&self, email: String) -> Result<Vec<Post>, Error> {
	// 	// println!("{}", email);
	// 	let q = r#"
	// 	SELECT *
	// 	FROM posts p
	// 	WHERE p.author_id IN (
	// 		SELECT id
	// 		FROM authors
	// 		WHERE email = $1
	// 	)
	// 	"#;

	// 	let record = sqlx::query_as::<_, Post>(q);

	// 	let post = record
	// 	.bind(email)
	// 	.fetch_all(&self.pool)
	// 	.await?;

	// 	Ok(post)
	// }

	// endregion: --Database Manipulations for posts
}


impl AppState {
	// #[tokio::main]
	/// Update the cache of authors
	pub async fn update_authors_cache(&self) {
		if let Ok(authors) = self.get_all_authors().await {

			update_cached_authors(&authors).await.expect("Failed to update cached authors");
			debug!(" {:<12 } - Cached Authors updated", "CACHE");

		} else {
			debug!("{:<12} Cache update failed", "AUTHORS")
		}
	}

	
	// #[tokio::main]
	/// Update the cache of posts
	pub async fn update_posts_cache(&self) {
		if let Ok(posts) = self.get_all_posts().await {

			update_cached_posts(&posts).await.expect("Failed to update cached posts");
			debug!(" {:<12 } - Cached Posts updated", "CACHE");

		} else {
			debug!("{:<12} Cache update failed", "POSTS")
		}
	}
}

