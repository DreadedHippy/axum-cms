use axum::{extract::State, Extension, Json};
use axum_extra::extract::WithRejection;

use crate::{models::{edit_suggestion::{EditSuggestion, EditSuggestionForCreate}, state::AppState}, utils::{auth::get_info_from_jwt, custom_extractor::ApiError}};
use crate::web::{custom_response::{CustomResponse, CustomResponseData}, error::{ServerError, ServerResult}};

pub async fn handler_edit_suggestion_create(
	State(app_state): State<AppState>,
	Extension(token): Extension<String>,
	WithRejection((Json(edit_suggestion_info)), _): WithRejection<Json<EditSuggestionForCreate>, ApiError>,
	) -> ServerResult<Json<CustomResponse<EditSuggestion>>>
	{
		
	let (_, author_id) = get_info_from_jwt(token)?;

		let result = app_state.create_edit_suggestion(edit_suggestion_info, author_id).await.map_err(|e| {
			ServerError::CouldNotCreateEditSuggestion
		})?;

		let response = CustomResponse::new(
			true,
			Some(format!("Edit suggestion created successfully")),
			Some(CustomResponseData::Item(result))
		);
	
		Ok(Json(response))
}