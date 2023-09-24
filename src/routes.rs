use axum::{Router, routing::{get, post}};

use crate::handlers::{hello::{handler_hello, handler_hello_2}, author::{handler_author_create, handler_author_get_all, handler_author_get_specific}, post::{handler_post_get_all, handler_post_create, handler_post_get_specific}};

pub fn all_routes() -> Router {
	Router::new()
		.merge(routes_hello())
		.merge(routes_author())
		.merge(routes_post())
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
		.route("/author/:id", get(handler_author_get_specific))
}

fn routes_post() -> Router {
	Router::new()
		.route(
			"/post",
			post(handler_post_create)
			.get(handler_post_get_all)
		)
		.route("/post/:id", get(handler_post_get_specific))
}