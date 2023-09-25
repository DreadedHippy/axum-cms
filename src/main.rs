#![allow(unused)]
use std::{net::SocketAddr, thread};

use anyhow::Result;
use axum::{Server, middleware};
use dotenv::dotenv;
use models::state::AppState;
use routes::all_routes;
use sqlx::{Pool, Postgres};
use utils::{main_response_mapper, connect_to_postgres};

mod routes;
mod handlers;
mod utils;
mod models;

#[tokio::main]
async fn main() -> Result<()>{    
    dotenv().ok();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Get Postgresql connection pool
    let pool = connect_to_postgres().await?;

    // Initialize App State with connection pool
    let app_state: AppState = AppState { pool };

    let all_routes = all_routes(app_state)
        .layer(middleware::map_response(main_response_mapper));

    // Start the server
    Server::bind(&addr)
        .serve(all_routes.into_make_service())
        .await
        .unwrap();

    Ok(())
}
