#![allow(unused)]
use std::net::SocketAddr;

use anyhow::Result;
use axum::{Server, middleware};
use routes::all_routes;
use utils::main_response_mapper;

mod routes;
mod handlers;
mod utils;
mod models;

#[tokio::main]
async fn main() -> Result<()>{    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let all_routes = all_routes()
        .layer(middleware::map_response(main_response_mapper));
    Server::bind(&addr)
        .serve(all_routes.into_make_service())
        .await
        .unwrap();

    Ok(())
}
