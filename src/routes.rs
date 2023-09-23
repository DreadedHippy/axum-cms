use axum::{Router, routing::{get, post}};

use crate::handlers::{hello::{handler_hello, handler_hello_2}, author::{handler_author_create, handler_author_get_all}};

pub fn all_routes() -> Router {
	Router::new()
		.merge(routes_hello())
		.merge(routes_author())
}


fn routes_hello() -> Router{
	Router::new()
		.route("/hello", get(handler_hello))
		.route("/hello2/:name", get(handler_hello_2))
}

fn routes_author() -> Router {
	Router::new()
		.route(
			"/author",
			post(handler_author_create)
			.get(handler_author_get_all)
		)
}