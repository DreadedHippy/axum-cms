use axum::{extract::State, Extension, Json};
use axum_extra::extract::WithRejection;
use tracing::debug;

use crate::{ctx::Ctx, models::{edit::{Edit, EditBmc, EditForCreate, EditForCreateRequestBody}, AppState}, web::{IncomingServerRequest, ServerResponse}};
use crate::web::{custom_response::{CustomResponse, CustomResponseData}, error::{ServerError, ServerResult}};
use crate::web::custom_extractor::ApiError;

pub async fn handler_edit_create(
	ctx: Ctx,
	State(app_state): State<AppState>,
	WithRejection((Json(edit_info)), _): IncomingServerRequest<EditForCreateRequestBody>,
	) -> ServerResponse<Edit>
	{
		
	debug!("{:<12} - handler_edit_create", "HANDLER");
	
	let editor_id = ctx.user_id();

	let data = EditForCreate {
		post_id: edit_info.post_id,
		new_content: edit_info.new_content,
		editor_id
	};

	let id = EditBmc::create(&ctx, &app_state, data).await?;

	let edit = EditBmc::get(&ctx, &app_state, id).await?;

	let response = CustomResponse::new(
		true,
		Some(format!("Edit created successfully")),
		Some(CustomResponseData::Item(edit))
	);

	Ok(Json(response))
}