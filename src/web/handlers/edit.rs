use axum::{extract::{Path, State}, http::StatusCode, Extension, Json};
use axum_extra::extract::WithRejection;
use serde_json::json;
use tracing::debug;

use crate::{ctx::Ctx, models::{edit::{Edit, EditBmc, EditFilter, EditForAccept, EditForCreate, EditForCreateRequestBody, EditForReject, EditForUpdate, EditForUpdateClientRequest, EditStatus}, post::{Post, PostBmc, PostFilter, PostForUpdate}, AppState}, web::{error::CrudError, IncomingServerRequest, ServerResponse}};
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

pub async fn handler_edit_get(
	ctx: Ctx,
	Path(id): Path<i64>,
	State(app_state): State<AppState>,
	) -> ServerResponse<Edit>
	{
		
	debug!("{:<12} - handler_edit_get", "HANDLER");
	
	// -- Retrieve info
	let user_id = ctx.user_id();
	let edit_id = id;
	let edit = EditBmc::get(&ctx, &app_state, edit_id).await?;
	let post = PostBmc::get(&ctx, &app_state, edit.post_id).await?;

	// -- Checks & Guards
	if edit.editor_id != user_id && post.author_id != user_id{
		let error = ServerError::GetFail(
			TABLE_NAME.to_string(),
			format!("You do not have permission to view this edit"),
			CrudError::UNAUTHORIZED
		);

		return Err(error)
	}

	// -- Build response
	let response = CustomResponse::new(
		true,
		Some(format!("Edit retrieved successfully")),
		Some(CustomResponseData::Item(edit))
	);

	Ok((StatusCode::OK, Json(response)))
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
	) -> ServerResponse<Edit>
	{
		
	debug!("{:<12} - handler_edit_list_outgoing", "HANDLER");

	// -- Retrieve info
	let user_id = ctx.user_id();

	let edit_filters: Vec<EditFilter> = serde_json::from_value(json!([
		{
			"editor_id": user_id
		}
	]))?;


	let edits = EditBmc::list(&ctx, &app_state, Some(edit_filters), None).await?;

	let response = CustomResponse::new(
		true,
		Some(format!("Outgoing edits retrieved successfully")),
		Some(CustomResponseData::Collection(edits))
	);

	Ok((StatusCode::OK, Json(response)))
	
}

pub async fn handler_edit_list_incoming(
	ctx: Ctx,
	State(app_state): State<AppState>,
	) -> ServerResponse<Edit>
	{
		
	debug!("{:<12} - handler_edit_list_incoming", "HANDLER");

	// -- Retrieve user id
	let user_id = ctx.user_id();

	// -- Get the ids of all posts authored by the querying user
	let post_filters: Vec<PostFilter> = serde_json::from_value(json!([
		{
			"author_id": user_id
		}
	]))?;

	let post_ids = PostBmc::list(&app_state, Some(post_filters), None).await?
		.iter()
		.map(|p| p.id).collect::<Vec<_>>()
	;


	// -- Get all edits where `post_id` is in the retrieved post ids
	let edit_filters: Vec<EditFilter> = serde_json::from_value(json!([
		{
			"post_id": {
				"$in": post_ids
			}
		}
	]))?;

	let edits = EditBmc::list(&ctx, &app_state, Some(edit_filters), None).await?;

	let response = CustomResponse::new(
		true,
		Some(format!("Incoming edits retrieved successfully")),
		Some(CustomResponseData::Collection(edits))
	);

	Ok((StatusCode::OK, Json(response)))
	
}

pub async fn handler_edit_list_all(
	ctx: Ctx,
	State(app_state): State<AppState>,
	) -> ServerResponse<Edit>
	{
		
	debug!("{:<12} - handler_edit_list_all", "HANDLER");

	// -- Retrieve user id
	let user_id = ctx.user_id();

	// -- Get the ids of all posts authored by the querying user
	let post_filters: Vec<PostFilter> = serde_json::from_value(json!([
		{
			"author_id": user_id
		}
	]))?;

	let post_ids = PostBmc::list(&app_state, Some(post_filters), None).await?
		.iter()
		.map(|p| p.id).collect::<Vec<_>>()
	;


	// -- Get all edits where `post_id` is in the retrieved post ids or `editor_id` corresponds to the querying user
	let edit_filters: Vec<EditFilter> = serde_json::from_value(json!([
		{
			"post_id": {
				"$in": post_ids
			}
		},
		{
			"editor_id": user_id
		}
	]))?;

	let edits = EditBmc::list(&ctx, &app_state, Some(edit_filters), None).await?;

	let response = CustomResponse::new(
		true,
		Some(format!("All edits retrieved successfully")),
		Some(CustomResponseData::Collection(edits))
	);

	Ok((StatusCode::OK, Json(response)))
	
}


pub async fn handler_edit_update(
	ctx: Ctx,
	State(app_state): State<AppState>,
	Path(id): Path<i64>,
	WithRejection((Json(data)), _): IncomingServerRequest<EditForUpdateClientRequest>
	) -> ServerResponse<Edit>
	{
		
	debug!("{:<12} - handler_edit_update", "HANDLER");

	// -- Retrieve info
	let user_id = ctx.user_id();
	let edit_id = id;

	// -- Checks & Guards
	let edit = EditBmc::get(&ctx, &app_state, edit_id).await?;

	if edit.editor_id != user_id {

		let error = ServerError::UpdateFail(
			TABLE_NAME.to_string(),
			format!("You do not have permission to alter the content of this edit"),
			CrudError::UNAUTHORIZED
		);
		return Err(error);
	}

	if edit.status != EditStatus::PENDING {

		let error = ServerError::UpdateFail(
			TABLE_NAME.to_string(),
			format!("The contents of this edit cannot be altered as it has already been {}", edit.status),
			CrudError::BAD_REQUEST
		);
		return Err(error);
	}
	
	// -- Update edit
	let edit_u = EditForUpdate {
		new_content: Some(data.new_content),
		status: None
	};

	let _result = EditBmc::update(&ctx, &app_state, edit_id, edit_u).await?;
	
	let edit = EditBmc::get(&ctx, &app_state, edit_id).await?;

	
	// -- Build response
	let response = CustomResponse::new(
		true,
		Some(format!("Edit updated successfully")),
		Some(CustomResponseData::Item(edit))
	);

	Ok((StatusCode::OK, Json(response)))
	
}

pub async fn handler_edit_delete(
	ctx: Ctx,
	State(app_state): State<AppState>,
	Path(id): Path<i64>,
	) -> ServerResponse<Edit>
	{
		
	debug!("{:<12} - handler_edit_delete", "HANDLER");

	// -- Retrieve info
	let user_id = ctx.user_id();
	let edit_id = id;
	let edit = EditBmc::get(&ctx, &app_state, edit_id).await?;

	// -- Checks & Guards

	if edit.editor_id != user_id {
		let error = ServerError::DeleteFail(
			TABLE_NAME.to_string(),
			format!("You do not have permission to delete this edit"),
			CrudError::UNAUTHORIZED
		);
		return Err(error);
	}

	if edit.status == EditStatus::ACCEPTED {
		let error = ServerError::UpdateFail(
			TABLE_NAME.to_string(),
			format!("This edit cannot be deleted as it has already been {}", edit.status),
			CrudError::BAD_REQUEST
		);
		return Err(error);
	}
	
	// -- Update edit
	let _result = EditBmc::delete(&ctx, &app_state, edit_id).await?;
	
	
	// -- Build response
	let response = CustomResponse::new(
		true,
		Some(format!("Edit deleted successfully")),
		None
	);

	Ok((StatusCode::OK, Json(response)))
	
}