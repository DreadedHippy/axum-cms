use crate::crypt::{pwd, EncryptContent};
use crate::ctx::Ctx;
use crate::models::AppState;
use crate::models::{ModelResult, ModelError};
use crate::models::base::{self, DbBmc};
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query, SimpleExpr};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use modql::field::{Fields, HasFields};
use sqlx::postgres::PgRow;
use sqlx::FromRow;
use uuid::Uuid;

// region:    --- Author Types

#[derive(Deserialize, Serialize, Debug, FromRow, Clone, Fields)]
///? [DEPRECATED] Complete Author model, as-is in the database
pub struct Author {
	pub id: i64,
	pub name: String,
	pub email: String,
	// pub password: String
}

#[derive(Deserialize, Debug, Fields)]
/// Struct holding fields required from client to create an author in the database
pub struct AuthorForCreate {
	pub name: String,
	pub email: String,
	pub password: String
}

#[derive(Deserialize, Debug, Fields)]
/// Struct holding fields required from client to create an author in the database
struct AuthorForInsert {
	email: String,
}

#[derive(Deserialize, Fields)]
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

#[derive(Clone, FromRow, Fields, Debug)]
pub struct AuthorForLogin { //? For login logic
	pub id: i64,
	pub email: String,

	// -- password and token info
	pub password: Option<String>, // encrypted, #_scheme_id_#...
	pub password_salt: Uuid,
	pub token_salt: Uuid,
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct AuthorForAuth { //? For authentication logic
	pub id: i64,
	pub email: String,

	// -- token info
	pub token_salt: Uuid,
}

/// Marker trait
pub trait AuthorBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl AuthorBy for Author {}
impl AuthorBy for AuthorForLogin {}
impl AuthorBy for AuthorForAuth {}

#[derive(Iden)]
enum AuthorIden {
	Id,
	Email,
	Password
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
	}

	pub async fn get<E>(
		ctx: &Ctx,
		app_state: &AppState,
		id: i64,
	) -> ModelResult<E>
	where
		E: AuthorBy
	{
		base::get::<Self, _>(ctx, app_state, id).await // Underscore on the second generic parameter because we return a model of author, the compiler can infer
	}

	pub async fn first_by_email<E>(
		ctx: &Ctx,
		app_state: &AppState,
		email: &str,
	) -> ModelResult<Option<E>>
	where
		E: AuthorBy
	{
		let db = app_state.db();

		// -- Build query

		let mut query = Query::select();

		query
			.from(Self::table_ref())
			.columns(E::field_idens())
			.and_where(Expr::col(AuthorIden::Email).eq(email));

		// -- Execute query
		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let author = sqlx::query_as_with::<_, E, _>(&sql, values)
			.fetch_optional(db)
			.await?;

		Ok(author)
		
	}

	pub async fn update_pwd(
		ctx: &Ctx,
		app_state: &AppState,
		id: i64,
		pwd_clear: &str
	) -> ModelResult<()> {
		let db = app_state.db();

		// -- Prep password
		let author: AuthorForLogin = Self::get(ctx, app_state, id).await?;
		let password = pwd::encrypt_pwd(&EncryptContent {
			content: pwd_clear.to_string(),
			salt: author.password_salt.to_string()
		})?;

		// -- Build query
		let mut query = Query::update();
		query
			.table(Self::table_ref())
			.value(AuthorIden::Password, SimpleExpr::from(password))
			.and_where(Expr::col(AuthorIden::Id).eq(id));

		// -- Execute query
		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let _count = sqlx::query_with(&sql, values)
			.execute(db)
			.await?
			.rows_affected();

		Ok(())
	}

	pub async fn list(ctx: &Ctx, app_state: &AppState) -> ModelResult<Vec<Author>> {
		base::list::<Self, _>(ctx, app_state).await // Underscore on the second generic parameter because we return a model of author, the compiler can infer
	}

	pub async fn update(ctx: &Ctx, app_state: &AppState, id: i64, author_e: AuthorForEdit) -> ModelResult<()> {
		base::update::<Self, _>(ctx, app_state, id, author_e).await
	}


	pub async fn delete(
		ctx: &Ctx,
		app_state: &AppState,
		id: i64,
	) -> ModelResult<()> {
		base::delete::<Self>(ctx, app_state, id).await
	}

}
// endregion: --- AuthorBmc

// region:    --- Tests
#[cfg(test)]
mod tests {
	#![allow(unused)]
	use crate::_dev_utils;

use super::*;
use anyhow::{Context, Ok, Result};
use serial_test::serial;

#[serial]
#[tokio::test]
	// async fn test_create_ok() -> Result<()> {
	// 	// -- Setup & Fixtures
	// 	let app_state = _dev_utils::init_test().await;
	// 	let ctx = Ctx::root_ctx();
	// 	let fx_title = "test_create_ok title";
		
	// 	// -- Exec
	// 	let author_c = AuthorForCreate {
	// 		name: fx_title.to_string(),
	// 		email: "email@e.mail".to_string(),
	// 		password: "welcome123".to_string()
	// 	};

	// 	let id = AuthorBmc::create(&ctx, &app_state, author_c).await?;

	// 	// -- Check
	// 	let author = AuthorBmc::get(&ctx, &app_state, id).await?;

	// 	assert_eq!(author.name, fx_title);

	// 	// -- Clean
	// 	AuthorBmc::delete(&ctx, &app_state, id).await?;
		
	// 	Ok(())
	// }

	// #[serial]
	// #[tokio::test]
	// async fn test_get_err_not_found() -> Result<()> {
	// 	// -- Setup & Fixtures
	// 	let app_state = _dev_utils::init_test().await;
	// 	let ctx = Ctx::root_ctx();
	// 	let fx_id = 100;

	// 	// -- Exec
	// 	let res = AuthorBmc::get(&ctx, &app_state, fx_id).await;

	// 	// println!("{:?}", res);
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
	
	// #[serial]
	// #[tokio::test]
	// async fn test_list_ok() -> Result<()> {
	// 	// -- Setup & Fixtures
	// 	let app_state = _dev_utils::init_test().await;
	// 	let ctx = Ctx::root_ctx();
	// 	let fx_authors = &[("test_list_ok-author 01", "email@1", "password1"), ("test_list_ok-author 02", "email@2", "password2")];
	// 	_dev_utils::seed_authors(&ctx, &app_state, fx_authors).await?;

	// 	// -- Exec
	// 	let authors = AuthorBmc::list(&ctx, &app_state).await?;
	// 	// println!("{:?}", authors);

		
	// 	let authors: Vec<Author> = authors
	// 	.into_iter()
	// 	.filter(|a| a.name.starts_with("test_list_ok-author"))
	// 	.collect();

	// 	assert_eq!(authors.len(), 2, "number of authors");

	// 	// -- Clean
	// 	for author in authors.iter() {
	// 		AuthorBmc::delete(&ctx, &app_state, author.id).await?;
	// 	}

	// 	Ok(())
	// }
	

	// #[serial]
	// #[tokio::test]
	// async fn test_delete_err_not_found() -> Result<()> {
	// 	// -- Setup & Fixtures
	// 	let app_state = _dev_utils::init_test().await;
	// 	let ctx = Ctx::root_ctx();
	// 	let fx_id = 100;

	// 	// -- Exec
	// 	let res = AuthorBmc::delete(&ctx, &app_state, fx_id).await;
	// 	// println!("{:?}", res);

	// 	assert!(
	// 		matches!(
	// 			res,
	// 			Err(ModelError::EntityNotFound {
	// 				entity: "authors",
	// 				id: fx_ud
	// 			})
	// 		),
	// 		"EntityNotFound not matching"
	// 	);

	// 	Ok(())
	// }

	// #[serial]
	// #[tokio::test]
	// async fn test_update_ok() -> Result<()> {
	// 	// -- Setup & Fixtures
	// 	let app_state = _dev_utils::init_test().await;
	// 	let ctx = Ctx::root_ctx();
	// 	let fx_info = ("test_update_ok - task 01", "email@1", "password");
	// 	let fx_info_new = "test_update_ok - task 01 - new ";

	// 	let fx_author = _dev_utils::seed_authors(&ctx, &app_state, &[fx_info])
	// 		.await?
	// 		.remove(0);

	// 	AuthorBmc::update(
	// 		&ctx,
	// 		&app_state,
	// 		fx_author.id,
	// 		AuthorForEdit {
	// 			name: Some(fx_info_new.to_string()),
	// 		}
	// 	).await?;

	// 	// -- Check
	// 	let author = AuthorBmc::get(&ctx, &app_state, fx_author.id).await?;
	// 	assert_eq!(author.name, fx_info_new);

	// 	Ok(())
	// }

	async fn test_first_ok_demo1() -> Result<()> {
			// -- Setup & Fixtures
			let app_state = _dev_utils::init_test().await;
			let ctx = Ctx::root_ctx();
			let fx_email = "e@mail";

			let author: Author = AuthorBmc::first_by_email(&ctx, &app_state, fx_email)
				.await?
				.context("Should have user 'Genesis'")?;

			assert_eq!(author.email, fx_email);

			Ok(())
	
	}
}
// endregion: --- Tests