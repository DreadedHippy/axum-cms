use sqlx::{Pool, Postgres, Error, Row};

use crate::models::author::{Author, AuthorForResult};

use super::{author::AuthorForCreate, post::{PostForCreate, Post}};

#[derive(Clone)]
pub struct AppState {
	pub pool: Pool<Postgres>
}

impl AppState {
	// region: --Database Manipulations for authors
	
	pub async fn create_author(&self, author_info: AuthorForCreate) -> Result<AuthorForResult, Error>{
		let q = r#"
		INSERT INTO authors (name, email, password)
		VALUES( $1, $2, $3)
		RETURNING *
		"#;

		let rec = sqlx::query(q)
		.bind(author_info.name)
		.bind(author_info.email)
		.bind(author_info.password)
		.fetch_one(&self.pool)
		.await?;

		let result = AuthorForResult {
			name: rec.get("name"),
			email: rec.get("email")
		};

		println!("{:#?}", result);

    Ok(result)
		
	}

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
	
	pub async fn create_post(&self, post_info: PostForCreate) -> Result<Post, Error>{
		let q = r#"
		INSERT INTO posts (title, content, author_id)
		VALUES( $1, $2, $3)
		RETURNING *
		"#;

		let rec = sqlx::query(q)
		.bind(post_info.title)
		.bind(post_info.content)
		.bind(post_info.author_id)
		.fetch_one(&self.pool)
		.await?;

		let result = Post {
			id: rec.get("id"),
			title: rec.get("title"),
			content: rec.get("content"),
			author_id: rec.get("author_id")
		};

		println!("{:#?}", result);

    Ok(result)		
	}

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

	pub async fn get_posts_by_author(&self, email: String) -> Result<Vec<Post>, Error> {
		println!("{}", email);
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