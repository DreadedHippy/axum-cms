use axum::{response::{IntoResponse, Html}, extract::{Query, Path}};
use tracing::debug;

use crate::web::HelloParams;

pub async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse{
	debug!("{:<12} - handler_hello - {params:?}", "HANDLER");
	let name = params.name.as_deref().unwrap_or("World");
	Html(format!("Hello <strong>{name}</strong>"))
}

pub async fn handler_hello_2(Path(name): Path<String>) -> impl IntoResponse {
	debug!("{:<12} - handler_hello_2 - {name:?}", "HANDLER");
	Html(format!("Hello from handler 2, Nice to meet you <strong><em>{name}</em></strong>"))
}