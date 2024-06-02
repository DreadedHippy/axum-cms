use axum::{handler::HandlerWithoutStateExt, http::StatusCode, routing::{any_service, MethodRouter}};
use tower_http::services::ServeDir;

use crate::config;

// region: Routes Static
pub fn serve_dir() -> MethodRouter {
	async fn handle_404() -> (StatusCode, &'static str) {
		(StatusCode::NOT_FOUND, "Resource not found")
	}

	any_service(
		ServeDir::new(&config().WEB_FOLDER)
			.not_found_service(handle_404.into_service())
	)
}