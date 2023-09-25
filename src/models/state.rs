use sqlx::{Pool, Postgres, Error, Row};

use crate::models::author::Author;

use super::author::AuthorForCreate;

#[derive(Clone)]
pub struct AppState {
	pub pool: Pool<Postgres>
}

impl AppState {
	// region: --Database Manipulations
	
	pub async fn create_author(&self, author_info: AuthorForCreate) -> Result<Author, Error>{
		let q = r#"
		INSERT INTO authors (name, email)
		VALUES( $1, $2)
		RETURNING *
		"#;

		let rec = sqlx::query(q)
		.bind(author_info.name)
		.bind(author_info.email)		
		.fetch_one(&self.pool)
		.await?;

		// let id: i64 = rec.get("id");
		// println!("{}" ,id);
		let result = Author {
			id: rec.get("id"),
			name: rec.get("name"),
			email: rec.get("email")
		};

		println!("{:#?}", result);

    Ok(result)
		
	}

	// endregion: --Database Manipulations
}