use crate::ctx::Ctx;
use crate::models::AppState;
use crate::models::ModelResult;
use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::FromRow;

use super::base;
use super::base::DbBmc;
use super::base::SqlField;
use super::base::SqlFields;
use super::ModelError;


// region:    --- Author Types

#[derive(Deserialize, Serialize, Debug, FromRow, Clone, Fields)]
/// Complete Author model, as-is in the database
pub struct Author {
	pub id: i64,
	pub name: String,
	pub email: String,
	pub password: String
}

#[derive(Deserialize, Debug, FromRow)]
/// Struct holding fields required from client to create an author in the database
pub struct AuthorForCreate {
	pub name: String,
	pub email: String,
	pub password: String
}

impl SqlFields for AuthorForCreate {
	fn to_field_value_pairs(&self) -> Vec<base::SqlField> {
			return vec![
				SqlField {
					name: "name",
					value: base::SqlFieldValue::String(self.name.clone())
				},
				SqlField {
					name: "email",
					value: base::SqlFieldValue::String(self.email.clone())
				},
				SqlField {
					name: "password",
					value: base::SqlFieldValue::String(self.password.clone())
				}
			]
	}
}
#[derive(Deserialize)]
/// Struct holding fields required from client to edit an author
pub struct AuthorForEdit {
	pub name: Option<String>,
}

#[derive(Serialize, Debug)]
/// Struct holding fields to be sent to the client as a resulting Author
pub struct AuthorForResult {
	pub id: i64,
	pub name: String,
	pub email: String
}

impl From<Author> for AuthorForResult {
	fn from(a: Author) -> Self{
		Self {
			id: a.id,
			name: a.name,
			email: a.email
		}
	}
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

// endregion: --- Author Types

// region:    --- AuthorBmc
pub struct AuthorBmc;

impl DbBmc for AuthorBmc {
	const TABLE: &'static str = "authors";
}

impl AuthorBmc {
	pub async fn create(
		ctx: &Ctx,
		app_state: &AppState,
		data: AuthorForCreate,
	) -> ModelResult<i64> {
		let db = app_state.db();

		base::create::<AuthorBmc, _>(ctx, app_state, data).await

		// let (id, ) = sqlx::query_as::<_, (i64,)>(
		// 	r#"
		// 		INSERT INTO authors (name, email, password)
		// 		VALUES( $1, $2, $3)
		// 		RETURNING id
		// 	"#
		// )
		// .bind(author_c.name)
		// .bind(author_c.email)
		// .bind(author_c.password)
		// .fetch_one(db)
		// .await?;

    // Ok(id)
		
	}

	pub async fn get(
		ctx: &Ctx,
		app_state: &AppState,
		id: i64,
	) -> ModelResult<Author> {
		base::get::<Self, _>(ctx, app_state, id).await // Underscore on the second generic parameter because we return a model of author, the compiler can infer
	}

	
	pub async fn list(_ctx: &Ctx, app_state: &AppState) -> ModelResult<Vec<Author>> {
		let db = app_state.db();

		let authors: Vec<Author> = sqlx::query_as(
			r#"
				SELECT * FROM authors ORDER BY id
			"#
		)
		.fetch_all(db)
		.await?;

    Ok(authors)
		
	}


	pub async fn delete(
		_ctx: &Ctx,
		app_state: &AppState,
		id: i64,
	) -> ModelResult<()> {
		let db = app_state.db();

		let count = sqlx::query("DELETE from authors WHERE id = $1")
		.bind(id)
		.execute(db)
		.await?
		.rows_affected();
	
		if count == 0 {
			return Err(ModelError::EntityNotFound { entity: "authors", id });
		}
		
    Ok(())
		
	}

}
// endregion: --- AuthorBmc

// region:    --- Tests
#[cfg(test)]
mod tests {
	#![allow(unused)]
	use crate::_dev_utils;

use super::*;
use anyhow::{Ok, Result};
use serial_test::serial;

#[tokio::test]
#[serial]
	async fn test_create_ok() -> Result<()> {
		// -- Setup & Fixtures
		let app_state = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_title = "test_create_ok title";
		
		// -- Exec
		let author_c = AuthorForCreate {
			name: fx_title.to_string(),
			email: "email@e.mail".to_string(),
			password: "welcome123".to_string()
		};

		let id = AuthorBmc::create(&ctx, &app_state, author_c).await?;

		// -- Check
		let author = AuthorBmc::get(&ctx, &app_state, id).await?;

		assert_eq!(author.name, fx_title);

		// -- Clean
		AuthorBmc::delete(&ctx, &app_state, id).await?;
		
		Ok(())
	}

	#[tokio::test]
	#[serial]
	async fn test_get_err_not_found() -> Result<()> {
		// -- Setup & Fixtures
		let app_state = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_id = 100;

		// -- Exec
		let res = AuthorBmc::get(&ctx, &app_state, fx_id).await;

		assert!(
			matches!(
				res,
				Err(ModelError::EntityNotFound {
					entity: "authors",
					id: 100
				})
			),
			"EntityNotFound not matching"
		);

		Ok(())
	}
	
	#[tokio::test]
	#[serial]
	async fn test_list_ok() -> Result<()> {
		// -- Setup & Fixtures
		let app_state = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_authors = &[("test_list_ok-author 01", "email@1", "password1"), ("test_list_ok-author 02", "email@2", "password2")];
		_dev_utils::seed_authors(&ctx, &app_state, fx_authors).await?;

		// -- Exec
		let authors = AuthorBmc::list(&ctx, &app_state).await?;

		
		let authors: Vec<Author> = authors
		.into_iter()
		.filter(|a| a.name.starts_with("test_list_ok-author"))
		.collect();

		assert_eq!(authors.len(), 2, "number of authors");

		// -- Clean
		for author in authors.iter() {
			AuthorBmc::delete(&ctx, &app_state, author.id).await?;
		}

		Ok(())
	}
	

	// #[tokio::test]
	// #[serial]
	// async fn test_delete_err_not_found() -> Result<()> {
		// -- Setup & Fixtures
	// 	let app_state = _dev_utils::init_test().await;
	// 	let ctx = Ctx::root_ctx();
	// 	let fx_id = 100;

	// 	// -- Exec
	// 	let res = AuthorBmc::delete(&ctx, &app_state, fx_id).await;
	// 	assert!(
	// 		matches!(
	// 			res,
	// 			Err(ModelError::EntityNotFound {
	// 				entity: "authors",
	// 				id: 100 
	// 			})
	// 		),
	// 		"EntityNotFound not matching"
	// 	);

	// 	Ok(())
	// }
}
// endregion: --- Tests