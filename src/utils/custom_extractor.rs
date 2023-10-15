use axum::{extract::rejection::JsonRejection, response::IntoResponse, Json};
use serde_json::json;
use thiserror::Error;
use tracing::debug;

#[derive(Debug, Error)]

/// Struct to help with JSON Deserialization error
pub enum ApiError {
	#[error(transparent)]
	JsonExtractorRejection(#[from] JsonRejection),
}

/// Implement IntoResponse to enable sending a server response
impl IntoResponse for ApiError {
	fn into_response(self) -> axum::response::Response {

		// Get the json rejection status and error
		let (status, message) = match self {
			ApiError::JsonExtractorRejection(json_rejection) => {
				(json_rejection.status(), json_rejection.body_text())
			}
		};

		// Create a JSON with it
		let payload = json!({
			"message": message,
			"origin": "with_rejection"
		});

		debug!(" {:<12} - json_rejection", "EXTRACTOR");
		// Send this JSON as the error response
		(status, Json(payload)).into_response()
	}
}