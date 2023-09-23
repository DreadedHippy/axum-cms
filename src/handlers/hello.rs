use axum::{response::{IntoResponse, Html}, extract::{Query, Path}};

use crate::models::HelloParams;

pub async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse{
	println!("-->> {:<12} - handler_hello - {params:?}", "HANDLER");
	let name = params.name.as_deref().unwrap_or("World");
	Html(format!("Hello <strong>{name}</strong>"))
}

pub async fn handler_hello_2(Path(name): Path<String>) -> impl IntoResponse {
	println!("-->> {:<12} - handler_hello_2 - {name:?}", "HANDLER");
	Html(format!("Hello from handler 2, Nice to meet you <strong><em>{name}</em></strong>"))
}