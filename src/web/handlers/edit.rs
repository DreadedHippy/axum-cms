use axum::{extract::{Path, State}, http::StatusCode, Extension, Json};
use axum_extra::extract::WithRejection;
use tracing::debug;

use crate::{ctx::Ctx, models::{edit::{Edit, EditBmc, EditForCreate, EditForCreateRequestBody, EditStatus}, AppState}, web::{error::CrudError, IncomingServerRequest, ServerResponse}};
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

	Ok((StatusCode::CREATED, Json(response)))
}

// pub async fn handler_edit_accept(
// 	ctx: Ctx,
// 	State(app_state): State<AppState>,
// 	Path(id): Path<i64>,
// 	WithRejection((Json(data)), _): IncomingServerRequest<Edit>,
// 	) -> ServerResponse<Edit>
// 	{
		
// 	debug!("{:<12} - handler_edit_create", "HANDLER");

// 	let user_id = ctx.user_id();
// 	let edit: Edit = EditBmc::get(&ctx, &app_state, id).await?;

// 	if edit.editor_id != user_id {
// 		let error = ServerError::UpdateFail("EDIT".to_string(), "Could not alter the status of this edit".to_string(), CrudError::FORBIDDEN);
// 		return Err(error);
// 	}

// 	if edit.status != EditStatus::PENDING {
// 		let response = CustomResponse::new(
// 			true,
// 			Some(format!("Edit created successfully")),
// 			Some(CustomResponseData::Item(edit))
// 		);
	
// 		Ok(Json(response))
// 	}
	
// }