use modql::{field::Fields, filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString}};
use serde_with::serde_as;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;
// use serial_test::*;

use crate::ctx::Ctx;

use super::{base::{self, DbBmc}, AppState, ModelResult};


// #[derive(Deserialize, Serialize, Debug, FromRow, Fields)]
#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize)]
/// Complete Post model, as-is in the database
pub struct Post {
	pub id: i64,
	pub title: String,
	pub content: String,
	pub author_id: i64,
	#[serde_as(as = "Rfc3339")]
	pub created_at: OffsetDateTime,
	#[serde_as(as = "Rfc3339")]
	pub updated_at: OffsetDateTime
}

#[derive(Deserialize, Debug, Fields)]
/// Struct holding fields required from client to create a post in the database
pub struct PostForCreate {
	pub title: String,
	pub content: String,
	pub author_id: i64
}

#[derive(Deserialize, Debug, Fields)]
/// Struct holding fields required from client to edit a post
pub struct PostForUpdate {
	pub title: Option<String>,
	pub content: Option<String>
}
#[derive(FilterNodes, Deserialize, Default)]
pub struct PostFilter {
	id: Option<OpValsInt64>,

	title: Option<OpValsString>,
	content: Option<OpValsString>,
	author_id: Option<OpValsInt64>
}

pub struct PostBmc;

impl DbBmc for PostBmc {
	const TABLE: &'static str = "posts";
}

impl PostBmc {
	pub async fn create(
		ctx: &Ctx,
		app_state: &AppState,
		data: PostForCreate,
	) -> ModelResult<i64> {
		let db = app_state.db();
		base::create::<PostBmc, _>(ctx, app_state, data).await
	}

	pub async fn get(
		ctx: &Ctx,
		app_state: &AppState,
		id: i64,
	) -> ModelResult<Post> {
		base::get::<Self, _>(ctx, app_state, id).await // Underscore on the second generic parameter because we return a model of author, the compiler can infer
	}

	pub async fn list(
		app_state: &AppState,
		filters: Option<Vec<PostFilter>>,
		list_options: Option<ListOptions>
	) -> ModelResult<Vec<Post>> {
		base::list_no_auth::<Self, _, _>(app_state, filters, list_options).await
	}

	
	pub async fn update(ctx: &Ctx, app_state: &AppState, id: i64, post_e: PostForUpdate) -> ModelResult<()> {
		base::update::<Self, _>(ctx, app_state, id, post_e).await
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
// #[serial]
mod tests {
	#![allow(unused)]
	use crate::{_dev_utils, models::ModelError};

	use super::*;
	use anyhow::{Ok, Result};
	use serial_test::serial;

	#[serial]
	#[tokio::test]
	async fn test_create_ok() -> Result<()> {
		// -- Setup & Fixtures
		let app_state = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_title = "test_create_ok title";
		let fx_content = "test_create_ok content";
		
		// -- Exec
		let post_c = PostForCreate {
			title: fx_title.to_string(),
			content: fx_content.to_string(),
			author_id: 1000 // Seeded user id
		};

		let id = PostBmc::create(&ctx, &app_state, post_c).await?;

		// -- Check
		let post = PostBmc::get(&ctx, &app_state, id).await?;

		assert_eq!(post.title, fx_title);
		assert_eq!(post.content, fx_content);

		// -- Clean
		PostBmc::delete(&ctx, &app_state, id).await?;
		
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
		let res = PostBmc::get(&ctx, &app_state, fx_id).await;

		// println!("{:?}", res);
		assert!(
			matches!(
				res,
				Err(ModelError::EntityNotFound {
					entity: "posts",
					id: fx_id
				})
			),
			"EntityNotFound not matching"
		);

		Ok(())
	}
	
	#[serial]
	#[tokio::test]
	async fn test_list_ok() -> Result<()> {
		// -- Setup & Fixtures
		let app_state = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_posts = &[("test_list_ok-post 01", "conent 01", 1000), ("test_list_ok-post 02", "content", 1000)];
		_dev_utils::seed_posts(&ctx, &app_state, fx_posts).await?;

		// -- Exec
		let posts = PostBmc::list(&app_state, None, None).await?;
		// println!("{:?}", posts);

		
		let posts: Vec<Post> = posts
		.into_iter()
		.filter(|p| p.title.starts_with("test_list_ok-post"))
		.collect();

		assert_eq!(posts.len(), 2, "number of posts");

		// -- Clean
		for post in posts.iter() {
			PostBmc::delete(&ctx, &app_state, post.id).await?;
		}

		Ok(())
	}
	

	#[serial]
	#[tokio::test]
	async fn test_delete_err_not_found() -> Result<()> {
		// -- Setup & Fixtures
		let app_state = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_id = 100;

		// -- Exec
		let res = PostBmc::delete(&ctx, &app_state, fx_id).await;
		// println!("{:?}", res);

		assert!(
			matches!(
				res,
				Err(ModelError::EntityNotFound {
					entity: "posts",
					id: fx_id
				})
			),
			"EntityNotFound not matching"
		);

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_update_ok() -> Result<()> {
		// -- Setup & Fixtures
		let app_state = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_info = ("test_update_ok - task 01", "content 01", 1000);
		let fx_info_new = ("test_update_ok - task 01 - new", "content 01 - new");

		let fx_post = _dev_utils::seed_posts(&ctx, &app_state, &[fx_info])
			.await?
			.remove(0);

		PostBmc::update(
			&ctx,
			&app_state,
			fx_post.id,
			PostForUpdate {
				title: Some(fx_info_new.0.to_string()),
				content: Some(fx_info_new.1.to_string())
			}
		).await?;

		// -- Check
		let post = PostBmc::get(&ctx, &app_state, fx_post.id).await?;
		assert_eq!(post.title, fx_info_new.0);
		assert_eq!(post.content, fx_info_new.1);

		Ok(())
	}
}
// endregion: --- Tests


