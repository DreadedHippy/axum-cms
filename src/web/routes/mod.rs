use axum::{extract::State, http::StatusCode, middleware, response::IntoResponse, routing::{delete, get, patch, post}, Router};
use sqlx::{Pool, Postgres};
use tower_cookies::CookieManagerLayer;

use crate::web::handlers::{author::{handler_author_list, handler_author_get}, hello::{handler_hello, handler_hello_2}, post::{handler_post_create, handler_post_delete, handler_post_update}};
use crate::models::AppState;

use super::{handlers::{edit::handler_edit_create, post::{handler_post_get, handler_post_list}}, middlewares::auth::mw_ctx_require};

pub fn routes_main(app_state: AppState) -> Router {
	Router::new()
		.merge(routes_post(app_state.clone()))
		.merge(routes_author(app_state.clone()))
		.merge(routes_edit(app_state.clone()))
		// .nest("/edit-suggestion", router)
		// .merge(routes_edit_suggestion(app_state.clone()))

}

/// Handling of authors
fn routes_author(app_state: AppState) -> Router {
	Router::new()
		.route(
			"/author",
			get(handler_author_list)
		)
		.route("/author/:id",
			get(handler_author_get)
		)
		.with_state(app_state)
}

/// Handling of posts
fn routes_post(app_state: AppState) -> Router {
	Router::new()
		.route(
			"/post",
			post(handler_post_create)
			.route_layer(middleware::from_fn(mw_ctx_require))
		)
		.route(
			"/post",
			get(handler_post_list)
		)
		.route("/post/:id", 
			get(handler_post_get)
		)
		.route("/post/:id",
			patch(handler_post_update)
			.delete(handler_post_delete)
			.route_layer(middleware::from_fn(mw_ctx_require))
		)
		.with_state(app_state)
}

/// Handling of edits
fn routes_edit(app_state: AppState) -> Router {
	Router::new()
		.route(
			"/edit",
			post(handler_edit_create)
			.route_layer(middleware::from_fn(mw_ctx_require))
		)
		// .route(
		// 	"/post",
		// 	get(handler_post_list)
		// )
		// .route("/post/:id", 
		// 	get(handler_post_get)
		// )
		// .route("/post/:id",
		// 	patch(handler_post_update)
		// 	.delete(handler_post_delete)
		// 	.route_layer(middleware::from_fn(mw_ctx_require))
		// )
		.with_state(app_state)
}

/// 404 Route
pub async fn handler_404() -> impl IntoResponse {
	(StatusCode::NOT_FOUND, "Route not found")
}