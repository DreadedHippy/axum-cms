#![allow(unused)] // For early development
use std::{net::SocketAddr, thread, env};
use anyhow::Result;
use axum::{middleware, response::Html, routing::get, Router, Server};
use dotenv::dotenv;
use models::AppState;
use web::{handlers::routes_static, middlewares::auth::{mw_ctx_require, mw_ctx_resolve}, routes::routes_main, routes_login};
use sqlx::{PgPool, Pool, Postgres};
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;
use web::middlewares::res_map::main_response_mapper;

mod config;
mod crypt;
mod web;
mod utils;
mod models;
mod ctx;
mod log;
mod error;

pub mod _dev_utils;

pub use config::config;
pub use self::web::{ServerError, ServerResult};

use crate::web::routes::handler_404;

#[tokio::main]
async fn main() -> Result<()>{
    tracing_subscriber::fmt()
        .without_time() // For early local development.
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // -- FOR DEV ONLY
    _dev_utils::init_dev().await;

    // Initialize ModelManager.
    let app_state = AppState::new().await?;

    // -- Define Routes
    let routes_all = Router::new()
        .merge(routes_login::routes(app_state.clone()))
        .nest("/api", routes_main(app_state.clone()))
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(app_state.clone(), mw_ctx_resolve))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static::serve_dir());


    // region:    --- Start Server

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("{:<12} - {addr}\n", "LISTENING");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .expect("Could not start server, `Server::bind` failed");

    // endregion: --- Start Server

    Ok(())
}
