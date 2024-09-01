use axum::{extract::State, Extension, Json};
use axum_extra::extract::WithRejection;

use crate::{ctx::Ctx, models::{edit_suggestion::{EditSuggestion, EditSuggestionForCreate}, state::AppState}, utils::auth::get_info_from_jwt, web::{IncomingServerRequest, ServerResponse}};
use crate::web::{custom_response::{CustomResponse, CustomResponseData}, error::{ServerError, ServerResult}};
use crate::web::custom_extractor::ApiError;

pub async fn handler_edit_suggestion_create(
	ctx: Ctx,
	State(app_state): State<AppState>,
	WithRejection((Json(edit_suggestion_info)), _): IncomingServerRequest<EditSuggestionForCreate>,
	) -> ServerResponse<()>
	{


		let response = CustomResponse::new(
			true,
			None,
			None
		);
	
		Ok(Json(response))
}