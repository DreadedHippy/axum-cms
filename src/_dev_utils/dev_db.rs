use std::{fs, path::PathBuf, time::Duration};
use tracing::info;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::{ctx::Ctx, models::{author::{Author, AuthorBmc}, state::AppState}};

type Db = Pool<Postgres>;

//* NOTE: Hardcode to prevent deployed system db update;
const PG_DEV_POSTGRES_URL: &str = "postgres://postgres:postgres@localhost:5433/axum_cms";
const PG_DEV_APP_URL: &str = "postgres://app_user:dev_only_pwd@localhost:5433/app_db";

// sql files
const SQL_RECREATE_DB: &str = "sql/dev_initial/00-recreate-db.sql";
const SQL_DIR: &str = "sql/dev_initial";

const DEMO_PWD: &str = "password";


pub async fn init_dev_db() -> Result<(), Box<dyn std::error::Error>> {
	info!("{:<12} - init_dev_db()", "FOR DEV ONLY");

	// -- Create the app_db/app_user with the postgres user.
	{
		let root_db = new_db_pool(PG_DEV_POSTGRES_URL).await?;
		pexec(&root_db, SQL_RECREATE_DB).await?;
	}

	// -- Get sql files.
	let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
		.filter_map(|entry| entry.ok().map(|e| e.path()))
		.collect();

	paths.sort();

	// SQL Execute each file
	let app_db = new_db_pool(PG_DEV_APP_URL).await?;
	for path in paths {
		if let Some(path) = path.to_str() {
			let path = path.replace('\\', "/"); // for windows

			// Only take the .sql and skip the SQL_RECREATE_DB
			if path.ends_with(".sql") && path != SQL_RECREATE_DB {
				pexec(&app_db, &path).await?;
			}
		}
	}

	// -- Init model layer
	let app_state = AppState::new().await?;
	let ctx = Ctx::root_ctx();

	// -- Set genesis pwd
	let genesis_author: Author = AuthorBmc::first_by_email(&app_state, "e@mail")
		.await?
		.unwrap();

	AuthorBmc::update_pwd(&ctx, &app_state, genesis_author.id, DEMO_PWD).await?;

	info!("{:<12} - init_dev_db - set genesis pwd", "FOR-DEV-ONLY");


	
	Ok(())
}

async fn pexec(db: &Db, file: &str) -> Result<(), sqlx::Error> {
	info!("{:<12} - pexec: {file}", "FOR-DEV-ONLY");

	// -- Read file
	let content = fs::read_to_string(file)?; // works because sqlx::Error impls from std::io;

	// FIXME: Make the split more sql proof
	let sqls: Vec<&str> =  content.split(";").collect();

	for sql in sqls {
		sqlx::query(sql).execute(db).await?;
	}

	Ok(())
}

async fn new_db_pool(db_conn_url: &str) -> Result<Db, sqlx::Error> {
	PgPoolOptions::new()
		.max_connections(1)
		.acquire_timeout(Duration::from_millis(500))
		.connect(db_conn_url)
		.await
}