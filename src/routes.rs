use axum::{Router, routing::{get, post}, extract::State, middleware};
use sqlx::{Pool, Postgres};
use tower_cookies::CookieManagerLayer;

use crate::{handlers::{hello::{handler_hello, handler_hello_2}, post::{handler_post_get_all, handler_post_create, handler_post_get_specific, handler_post_edit}, auth::{handler_login, handler_signup}, author::{handler_author_get_all, handler_author_get_specific, handler_author_edit}}, models::state::AppState, middlewares::{self, cache::{mw_get_cached_posts, mw_get_cached_authors}}};

pub fn all_routes(app_state: AppState) -> Router {
	Router::new()
		// .merge(routes_hello(state.clone()))
		.merge(routes_author(app_state.clone()))
		.merge(routes_post(app_state.clone()))
		.merge(routes_auth(app_state.clone()))

}


// fn routes_hello() -> Router{
// 	Router::new()
// 		.route("/hello", get(handler_hello))
// 		.route("/hello2/:name", get(handler_hello_2))
// }

fn routes_author(app_state: AppState) -> Router {
	Router::new()
		.route(
			"/author",
			// post(handler_author_create).route_layer(middleware::from_fn(middlewares::auth::mw_require_auth))
			get(handler_author_get_all).route_layer(middleware::from_fn(mw_get_cached_authors))
		)
		.route("/author/:id",
			get(handler_author_get_specific)
			.patch(handler_author_edit)
		)
		.with_state(app_state)
}

fn routes_post(app_state: AppState) -> Router {
	Router::new()
		.route(
			"/post",
			post(handler_post_create)
			.get(handler_post_get_all).route_layer(middleware::from_fn(mw_get_cached_posts))
		)
		.route("/post/:id", 
			get(handler_post_get_specific)
			.patch(handler_post_edit)
		)
		.with_state(app_state)
}

fn routes_auth(app_state: AppState) -> Router {
	Router::new()
		.route(
			"/login", post(handler_login)
		)
		.route(
			"/signup", post(handler_signup)
		).with_state(app_state)
}