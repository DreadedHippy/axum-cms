#![allow(unused)]
use std::{net::SocketAddr, thread};

use anyhow::Result;
use axum::{Server, middleware};
use dotenv::dotenv;
use models::state::AppState;
use routes::all_routes;
use sqlx::{Pool, Postgres};
use tower_cookies::CookieManagerLayer;
use utils::{main_response_mapper, connect_to_postgres, cache::{create_redis_connection, initialize_cache}};

mod routes;
mod handlers;
mod utils;
mod models;
mod middlewares;

#[tokio::main]
async fn main() -> Result<()>{    
    dotenv().ok();
    // Declare host and port number
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Get Postgresql connection pool
    let pool = connect_to_postgres().await?;

    // Initialize App State with connection pool
    let app_state: AppState = AppState { pool };

    // Get Redis Client;
    let connection = create_redis_connection().await.unwrap();

    // Get information and initialize the cache
    let initial_authors = app_state.get_all_authors().await?;
    let initial_posts = app_state.get_all_posts().await?;

    initialize_cache(initial_authors, initial_posts);

    let all_routes = all_routes(app_state)
        .layer(middleware::map_response(main_response_mapper))
		.layer(CookieManagerLayer::new());

    
    println!("Axum server listening on port 3000");

    // Start the server
    Server::bind(&addr)
        .serve(all_routes.into_make_service())
        .await
        .unwrap();


    Ok(())
}
