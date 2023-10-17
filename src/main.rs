#![allow(unused)] // For early development
use std::{net::SocketAddr, thread, env};
use anyhow::Result;
use axum::{Server, middleware};
use dotenv::dotenv;
use models::state::AppState;
use routes::all_routes;
use sqlx::{Pool, Postgres};
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;
use utils::{main_response_mapper, connect_to_postgres, cache::{create_redis_connection, initialize_cache}};

mod config;
mod routes;
mod handlers;
mod utils;
mod models;
mod middlewares;

pub mod _dev_utils;

pub use config::config;

#[tokio::main]
async fn main() -> Result<()>{
    dotenv().ok();

    tracing_subscriber::fmt()
        .without_time() // For early local development
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // -- FOR DEV ONLY

    let (database_url) = match env::var("MODE") {
        Ok(mode) => {
            if mode == String::from("production") {
                tracing::warn!("PRODUCTION MODE");
                env::var("PROD_DATABASE_URL").expect("Env variable `PROD_DATABASE_URL` not found")
            } else {
                _dev_utils::init_dev().await;
                env::var("DEV_DATABASE_URL").expect("Env variable `DEV_DATABASE_URL` not found")
            }
        },
        _ => {
            _dev_utils::init_dev().await;
            env::var("DEV_DATABASE_URL").expect("Env variable `DEV_DATABASE_URL` not found")
        }
    };
    
    // Declare host and port number
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    // Get Postgresql connection pool
    let pool = connect_to_postgres(database_url).await?;
	info!("CONNECTED TO POSTGRES");

    // Initialize App State with connection pool
    let app_state: AppState = AppState { pool };

    // Get Redis Client;
    let connection = create_redis_connection().await.expect("Could not connect to redis");
	info!("CONNECTED TO REDIS");

    // Get information and initialize the cache
    let initial_authors = app_state.get_all_authors().await?;
    let initial_posts = app_state.get_all_posts().await?;

    initialize_cache(initial_authors, initial_posts).await;

    let all_routes = all_routes(app_state)
        .layer(middleware::map_response(main_response_mapper))
		.layer(CookieManagerLayer::new());

    
    info!("{:<12} - {addr}\n", "LISTENING");

    // Start the server
    Server::bind(&addr)
        .serve(all_routes.into_make_service())
        .await
        .expect("Could not start server `Server::bind` failed");


    Ok(())
}
