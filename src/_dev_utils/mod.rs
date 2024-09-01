mod dev_db;
use tokio::sync::OnceCell;
use tracing::info;

use crate::{ctx::Ctx, models::{author::{self, Author, AuthorBmc, AuthorForCreate}, post::{Post, PostBmc, PostForCreate}, state::AppState, ModelResult}};

/// Initialize environment for local development
pub async fn init_dev() {
	static INIT: OnceCell<()> = OnceCell::const_new();

	INIT.get_or_init(|| async {
		info!("{:<12} - init_dev_all()", "FOR-DEV-ONLY");

		dev_db::init_dev_db().await.expect("Could not initialize dev database");
	})
	.await;
}

/// Initialize test environment
pub async fn init_test() -> AppState {
	static INIT: OnceCell<AppState> = OnceCell::const_new();

	let a_s = INIT.get_or_init(|| async {
		init_dev().await;
		AppState::new().await.unwrap()
	})
	.await;

	a_s.clone()
}

pub async fn seed_authors(ctx: &Ctx, app_state: &AppState, authors: &[(&str, &str, &str)]) -> ModelResult<Vec<Author>>{
	let mut author_results = Vec::new();
	for author_c in authors {
		let  id = AuthorBmc::create(
			ctx,
			app_state,
			AuthorForCreate {
				name: author_c.0.to_string(),
				email: author_c.1.to_string(),
				password: author_c.2.to_string()
			}
		).await?;

		let author = AuthorBmc::get(ctx, app_state, id).await?;
		author_results.push(author);
	}

	Ok(author_results)

}

pub async fn seed_posts(ctx: &Ctx, app_state: &AppState, posts: &[(&str, &str, i64)]) -> ModelResult<Vec<Post>>{
	let mut post_results = Vec::new();
	for post_c in posts {
		let  id = PostBmc::create(
			ctx,
			app_state,
			PostForCreate {
				title: post_c.0.to_string(),
				content: post_c.1.to_string(),
				author_id: post_c.2,
			}
		).await?;

		let post = PostBmc::get(ctx, app_state, id).await?;
		post_results.push(post);
	}

	Ok(post_results)

}