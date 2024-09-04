use axum::{extract::{Path, State}, http::StatusCode, Extension, Json};
use axum_extra::extract::WithRejection;
use serde_json::json;
use tracing::debug;

use crate::{ctx::Ctx, models::{edit::{Edit, EditBmc, EditFilter, EditForAccept, EditForCreate, EditForCreateRequestBody, EditForReject, EditForUpdate, EditStatus}, post::{Post, PostBmc, PostFilter, PostForUpdate}, AppState}, web::{error::CrudError, IncomingServerRequest, ServerResponse}};
use crate::web::{custom_response::{CustomResponse, CustomResponseData}, error::{ServerError, ServerResult}};
use crate::web::custom_extractor::ApiError;

const TABLE_NAME: &str = "EDIT";
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

pub async fn handler_edit_accept(
	ctx: Ctx,
	State(app_state): State<AppState>,
	Path(id): Path<i64>,
	WithRejection((Json(data)), _): IncomingServerRequest<EditForAccept>,
	) -> ServerResponse<Edit>
	{
		
	debug!("{:<12} - handler_edit_accept", "HANDLER");

	// -- Retrieve info
	let user_id = ctx.user_id();
	let edit: Edit = EditBmc::get(&ctx, &app_state, id).await?;
	let post: Post = PostBmc::get(&ctx, &app_state, edit.post_id).await?;

	// -- Checks and guards
	if post.author_id != user_id {
		let error = ServerError::UpdateFail(TABLE_NAME.to_string(), "You do not have permission to alter the status of this edit".to_string(), CrudError::UNAUTHORIZED);
		return Err(error);
	}

	if !(data.accept) {
		let error = ServerError::UpdateFail(TABLE_NAME.to_string(), "`accept` set to false".to_string(), CrudError::BAD_REQUEST);
		return Err(error);
	}

	if edit.status != EditStatus::PENDING {
		let error = ServerError::UpdateFail(TABLE_NAME.to_string(), format!("Edit has already been {}",edit.status), CrudError::FORBIDDEN);
		return Err(error);
	}

	// -- Update Values

	// Update post content
	let post_u = PostForUpdate {
		content: Some(edit.new_content.clone()),
		title: None
	};

	let _result = PostBmc::update(&ctx, &app_state, edit.post_id, post_u).await?;	

	// Update edit status
	let edit_u = EditForUpdate {
		new_content: None,
		status: Some(EditStatus::ACCEPTED)
	};

	let _result = EditBmc::update(&ctx, &app_state, id, edit_u).await?;
	let edit = EditBmc::get(&ctx, &app_state, id).await?;

	let response = CustomResponse::new(
		true,
		Some(format!("Edit accepted successfully")),
		Some(CustomResponseData::Item(edit))
	);

	Ok((StatusCode::OK, Json(response)))
	
}

pub async fn handler_edit_reject(
	ctx: Ctx,
	State(app_state): State<AppState>,
	Path(id): Path<i64>,
	WithRejection((Json(data)), _): IncomingServerRequest<EditForReject>,
	) -> ServerResponse<Edit>
	{
		
	debug!("{:<12} - handler_edit_reject", "HANDLER");

	// -- Retrieve info
	let user_id = ctx.user_id();
	let edit: Edit = EditBmc::get(&ctx, &app_state, id).await?;
	let post: Post = PostBmc::get(&ctx, &app_state, edit.post_id).await?;

	// -- Checks and guards
	if post.author_id != user_id {
		let error = ServerError::UpdateFail(TABLE_NAME.to_string(), "You do not have permission to alter the status of this edit".to_string(), CrudError::UNAUTHORIZED);
		return Err(error);
	}

	if !(data.reject) {
		let error = ServerError::UpdateFail(TABLE_NAME.to_string(), "`reject` set to false".to_string(), CrudError::BAD_REQUEST);
		return Err(error);
	}

	if edit.status != EditStatus::PENDING {
		let error = ServerError::UpdateFail(TABLE_NAME.to_string(), format!("Edit has already been {}",edit.status), CrudError::FORBIDDEN);
		return Err(error);
	}

	// -- Update edit status
	let edit_u = EditForUpdate {
		new_content: None,
		status: Some(EditStatus::REJECTED)
	};

	let _result = EditBmc::update(&ctx, &app_state, id, edit_u).await?;
	let edit = EditBmc::get(&ctx, &app_state, id).await?;

	let response = CustomResponse::new(
		true,
		Some(format!("Edit rejected successfully")),
		Some(CustomResponseData::Item(edit))
	);

	Ok((StatusCode::OK, Json(response)))
	
}

pub async fn handler_edit_list_outgoing(
	ctx: Ctx,
	State(app_state): State<AppState>,
	) -> ServerResponse<Vec<Edit>>
	{
		
	debug!("{:<12} - handler_edit_list_outgoing", "HANDLER");

	// -- Retrieve info
	let user_id = ctx.user_id();

	let edit_filter: EditFilter = serde_json::from_value(json!({
		"editor_id": user_id
	}))?;


	let edits = EditBmc::list(&ctx, &app_state, Some(edit_filter), None).await?;

	let response = CustomResponse::new(
		true,
		Some(format!("Outgoing edits retrieved successfully")),
		Some(CustomResponseData::Item(edits))
	);

	Ok((StatusCode::OK, Json(response)))
	
}

pub async fn handler_edit_list_incoming(
	ctx: Ctx,
	State(app_state): State<AppState>,
	) -> ServerResponse<Vec<Edit>>
	{
		
	debug!("{:<12} - handler_edit_list_incoming", "HANDLER");

	// -- Retrieve user id
	let user_id = ctx.user_id();

	// -- Get the ids of all posts authored by the querying user
	let post_filter: PostFilter = serde_json::from_value(json!({
		"author_id": user_id
	}))?;

	let post_ids = PostBmc::list(&app_state, Some(post_filter), None).await?
		.iter()
		.map(|p| p.id).collect::<Vec<_>>()
	;


	// -- Get all edits where `post_id` is in the retrieved post ids
	let edit_filter: EditFilter = serde_json::from_value(json!({
		"post_id": {
			"$in": post_ids
		}
	}))?;

	let edits = EditBmc::list(&ctx, &app_state, Some(edit_filter), None).await?;

	let response = CustomResponse::new(
		true,
		Some(format!("Incoming edits retrieved successfully")),
		Some(CustomResponseData::Item(edits))
	);

	Ok((StatusCode::OK, Json(response)))
	
}