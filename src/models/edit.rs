use modql::{field::Fields, filter::{FilterGroups, FilterNodes, ListOptions, OpValsBool, OpValsInt64, OpValsString, OpValsValue}};
use sea_query::{error::Error, Alias, Condition, Expr, Iden, Nullable, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde_with::serde_as;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::Type, FromRow};

use crate::ctx::Ctx;

use super::{base::{self, DbBmc}, AppState, ModelError, ModelResult};

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize, FromRow, Fields)]
/// Complete "Edit" model as-is in the database
pub struct Edit {
	pub id: i64,
	pub editor_id: i64,
	pub post_id: i64,
	pub new_content: String,
	pub status: EditStatus,
	#[serde_as(as = "Rfc3339")]
	pub created_at: OffsetDateTime,
	#[serde_as(as = "Rfc3339")]
	pub updated_at: OffsetDateTime
}


/// Complete "Edit Status" enum as-is in the database
// #[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "edit_status")]
#[derive(Clone, Debug, Deserialize, strum_macros::Display, Serialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "edit_status")]
pub enum EditStatus {
	PENDING,
	ACCEPTED,
	REJECTED
}


impl From<EditStatus> for sea_query::Value {
	fn from(val: EditStatus) -> Self {
		val.to_string().into()
	}
}

impl Nullable for EditStatus {
	fn null() -> sea_query::Value {
		EditStatus::PENDING.into()
	}
}

#[derive(Deserialize, Debug, Fields)]
/// Struct holding fields required to create an edit suggestion in the database
pub struct EditForCreate {
	pub post_id: i64,
	pub new_content: String,
	pub editor_id: i64
}

#[derive(Deserialize, Debug, Fields)]
/// Struct holding fields required from client to create an edit suggestion in the database
pub struct EditForCreateRequestBody {
	pub post_id: i64,
	pub new_content: String,
}


#[derive(Deserialize, Debug, Fields)]
/// Struct holding fields required from client to edit an edit_suggestion
pub struct EditForUpdate {
	pub new_content: Option<String>,
	pub status: Option<EditStatus>,
}


#[derive(Serialize, Debug)]
/// Struct holding fields to be sent to the client as a resulting EditSuggestion
pub struct EditForResult {
	pub status: EditStatus,
	pub new_content: String
}

#[derive(FilterNodes, Deserialize, Default)]
pub struct EditFilter {
	id: Option<OpValsInt64>,

	editor_id: Option<OpValsInt64>,
	post_id: Option<OpValsInt64>,
	new_content: Option<OpValsString>,
	status: Option<OpValsValue>
}

#[derive(Deserialize, Debug)]
pub struct EditForAccept {
	pub accept: bool
}

#[derive(Deserialize, Debug)]
pub struct EditForReject {
	pub reject: bool
}

#[derive(Iden)]
enum EditIden {
	Id,
	PostId,
	EditorId,
	NewContent,
	Status
}


pub struct EditBmc;

impl DbBmc for EditBmc {
	const TABLE: &'static str = "edits";
}

impl EditBmc {
	pub async fn create(
		ctx: &Ctx,
		app_state: &AppState,
		data: EditForCreate,
	) -> ModelResult<i64> {
		let db = app_state.db();
		base::create::<EditBmc, _>(ctx, app_state, data).await
	}

	pub async fn get(
		ctx: &Ctx,
		app_state: &AppState,
		id: i64,
	) -> ModelResult<Edit> {
		base::get::<Self, _>(ctx, app_state, id).await // Underscore on the second generic parameter because we return a model of author, the compiler can infer
	}

	pub async fn list(ctx: &Ctx, app_state: &AppState, filters: Option<EditFilter>, list_options: Option<ListOptions>) -> ModelResult<Vec<Edit>> {
		base::list::<Self, _, _>(ctx, app_state, filters, list_options).await
	}
	
	pub async fn update(ctx: &Ctx, app_state: &AppState, id: i64, edit_e: EditForUpdate) -> ModelResult<()> {
		let db = app_state.db();

		let mut query = Query::update();

		query.table(Self::table_ref());

		if let Some(s) = edit_e.status {
			query.value(EditIden::Status, Expr::val(s).as_enum(Alias::new("edit_status")));
		}

		if let Some(c) = edit_e.new_content {
			query.value(EditIden::NewContent, c);
		}
		
		query.and_where(Expr::col(EditIden::Id).eq(id));

		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let count = sqlx::query_with(&sql, values)
			.execute(db)
			.await?
			.rows_affected();

		// -- Check result
		if count == 0 {
			Err(ModelError::EntityNotFound { entity: Self::TABLE, id })
		} else {
			Ok(())
		}
	}

	pub async fn delete(
		ctx: &Ctx,
		app_state: &AppState,
		id: i64,
	) -> ModelResult<()> {
		base::delete::<Self>(ctx, app_state, id).await
	}
}

// region:    --- Tests
#[cfg(test)]

mod tests {
	#![allow(unused)]
	use crate::{_dev_utils, models::{author::AuthorBmc, post::PostBmc, ModelError}};

	use super::*;
	use anyhow::{Ok, Result};
	use serial_test::serial;

	#[serial]
	#[tokio::test]
	async fn test_create_ok() -> Result<()> {
		// -- Setup & Fixtures
		let app_state = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_editors = &[("name", "email@mail", "password")];
		let fx_posts = &[("test_list_ok-post 01", "content 01", 1000)];

		_dev_utils::seed_posts(&ctx, &app_state, fx_posts).await?;
		_dev_utils::seed_authors(&ctx, &app_state, fx_editors).await?;

		let posts = PostBmc::list(&app_state, None, None).await?;
		let editors = AuthorBmc::list(&app_state, None, None).await?;
		let post = &posts[0];
		let editor = &editors[0];

		let fx_new_content = "Here is a suggestion";
		let fx_post_id = post.id;
		let fx_editor_id = editor.id;
		
		// -- Exec
		let edit_c = EditForCreate {
			new_content: fx_new_content.to_string(),
			post_id: fx_post_id,
			editor_id: fx_editor_id
		};

		
		// -- Check
		let id = EditBmc::create(&ctx, &app_state, edit_c).await?;
		let edit = EditBmc::get(&ctx, &app_state, id).await?;


		assert_eq!(edit.new_content, fx_new_content);
		assert_eq!(edit.post_id, fx_post_id);
		assert_eq!(edit.editor_id, fx_editor_id);

		// -- Clean
		EditBmc::delete(&ctx, &app_state, id).await?;
		
		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_get_err_not_found() -> Result<()> {
		// -- Setup & Fixtures
		let app_state = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_id = 100;

		// -- Exec
		let res = EditBmc::get(&ctx, &app_state, fx_id).await;

		// println!("{:?}", res);
		assert!(
			matches!(
				res,
				Err(ModelError::EntityNotFound {
					entity: "edits",
					id: fx_id
				})
			),
			"EntityNotFound not matching"
		);

		Ok(())
	}
}
// endregion: --- Tests