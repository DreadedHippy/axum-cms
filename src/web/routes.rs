use axum::{extract::State, http::StatusCode, middleware, response::IntoResponse, routing::{get, patch, post}, Router};
use sqlx::{Pool, Postgres};
use tower_cookies::CookieManagerLayer;

use crate::web::{handlers::{auth::{handler_login, handler_signup}, author::{handler_author_edit, handler_author_get_all, handler_author_get_specific}, edit_suggestion::handler_edit_suggestion_create, hello::{handler_hello, handler_hello_2}, post::{handler_post_create, handler_post_delete, handler_post_edit, handler_post_get_all, handler_post_get_specific}}, middlewares::{self, cache::{mw_get_cached_authors, mw_get_cached_posts}}};
use crate::models::state::AppState;

pub fn all_routes(app_state: AppState) -> Router {
	Router::new()
		// .merge(routes_hello(state.clone()))
		.merge(routes_author(app_state.clone()))
		.merge(routes_post(app_state.clone()))
		.merge(routes_auth(app_state.clone()))
		// .nest("/edit-suggestion", router)
		// .merge(routes_edit_suggestion(app_state.clone()))

}

/// Handling of authors
fn routes_author(app_state: AppState) -> Router {
	Router::new()
		.route(
			"/author",
			// post(handler_author_create).route_layer(middleware::from_fn(middlewares::auth::mw_require_auth))
			get(handler_author_get_all).route_layer(middleware::from_fn(mw_get_cached_authors))
		)
		.route("/author/:id",
			get(handler_author_get_specific)
		)
		// .route("/author/:id",
		// 	patch(handler_author_edit).route_layer(middleware::from_fn(mw_require_auth))
		// )
		.with_state(app_state)
}

/// Handling of posts
fn routes_post(app_state: AppState) -> Router {
	Router::new()
		// .route(
		// 	"/post",
		// 	post(handler_post_create).route_layer(middleware::from_fn(mw_require_auth))
		// )
		.route(
			"/post",
			get(handler_post_get_all).route_layer(middleware::from_fn(mw_get_cached_posts))
		)
		.route("/post/:id", 
			get(handler_post_get_specific)
		)
		// .route("/post/:id",
		// 	patch(handler_post_edit)
		// 	.delete(handler_post_delete).route_layer(middleware::from_fn(mw_require_auth))
		// )
		.with_state(app_state)
}

/// Handling of auth-related functionalities
fn routes_auth(app_state: AppState) -> Router {
	Router::new()
		.route(
			"/login", post(handler_login)
		)
		.route(
			"/signup", post(handler_signup)
		).with_state(app_state)
}

// fn routes_edit_suggestion(app_state: AppState) -> Router {
// 	Router::new()
// 		.route(
// 			"/edit-suggestion/new",
// 			post(handler_edit_suggestion_create).route_layer(middleware::from_fn(mw_require_auth))
// 		).with_state(app_state)
// }

/// 404 Route
pub async fn handler_404() -> impl IntoResponse {
	(StatusCode::NOT_FOUND, "Route not found")
}