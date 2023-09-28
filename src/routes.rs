use axum::{Router, routing::{get, post}, extract::State, middleware};
use sqlx::{Pool, Postgres};
use tower_cookies::CookieManagerLayer;

use crate::{handlers::{hello::{handler_hello, handler_hello_2}, post::{handler_post_get_all, handler_post_create, handler_post_get_specific}, auth::{handler_login, handler_signup}}, models::state::AppState, middlewares};

pub fn all_routes(app_state: AppState) -> Router {
	Router::new()
		// .merge(routes_hello(state.clone()))
		// .merge(routes_author(app_state.clone()))
		.merge(routes_post(app_state.clone()))
		.merge(routes_auth(app_state.clone()))

}


// fn routes_hello() -> Router{
// 	Router::new()
// 		.route("/hello", get(handler_hello))
// 		.route("/hello2/:name", get(handler_hello_2))
// }

// fn routes_author(app_state: AppState) -> Router {
// 	Router::new()
// 		.route(
// 			"/author",
// 			post(handler_author_create).route_layer(middleware::from_fn(middlewares::auth::mw_require_auth))
// 			.get(handler_author_get_all)
// 		)
// 		.route("/author/:id", get(handler_author_get_specific))
// 		.with_state(app_state)
// }

fn routes_post(app_state: AppState) -> Router {
	Router::new()
		.route(
			"/post",
			post(handler_post_create)
			.get(handler_post_get_all)
		)
		.route("/post/:id", get(handler_post_get_specific))
		.with_state(app_state)
}

fn routes_auth(app_state: AppState) -> Router {
	// #[axum::debug_handler]
	Router::new()
		.route(
			"/login", post(handler_login)
		)
		.route(
			"/signup", post(handler_signup)
		).with_state(app_state)
}