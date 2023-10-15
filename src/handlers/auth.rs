use axum::{Json, extract::State};
use axum_extra::extract::WithRejection;
use chrono::format::format;
use tower_cookies::{Cookies, Cookie};
use tracing::debug;

use crate::{models::{auth::LoginPayload, custom_response::{CustomResponse, CustomResponseData}, error::{Error, Result}, state::AppState, author::{AuthorForCreate, Author, AuthorForResult}}, middlewares::{AUTH_TOKEN, AUTHORIZATION_HEADER}, utils::{auth::{create_jwt, hash_password, verify_hash}, custom_extractor::ApiError}};

/// Handler to manage author login
pub async fn handler_login(
	cookies: Cookies,
	State(app_state): State<AppState>,
	WithRejection((Json(payload)), _): WithRejection<Json<LoginPayload>, ApiError>,
	) -> Result<Json<CustomResponse<String>>>{
	debug!(" {:<12} - api_login", "HANDLER");

	// Check for author in DB
	let author_from_db = app_state.get_author_by_email(payload.email).await.map_err(|_| Error::CouldNotGetAuthor)?;


	// Confirm password match
	if let Ok(false) = verify_hash(payload.password, &author_from_db.password) {
		return Err(Error::LoginFail)
	}
	
	// Create jwt
	let jwt = create_jwt(author_from_db.email.clone(), author_from_db.id)?;

	// Set auth header cookie
	cookies.add(Cookie::new(AUTHORIZATION_HEADER, format!("Bearer {}", jwt)));

	// Return successful message
	let response = CustomResponse::<String>::new(
		true,
		Some(format!("Logged in Successfully")),
		None
	);

	Ok(Json(response))
}

/// Handler to manage author sign-up
pub async fn handler_signup(
	cookies: Cookies,
	State(app_state): State<AppState>,
	WithRejection((Json(author_info)), _): WithRejection<Json<AuthorForCreate>, ApiError>
) -> Result<Json<CustomResponse<AuthorForResult>>> {
	// Hash the password
	let password = hash_password(author_info.password.clone())?;
	debug!(" {:<12} - api_signup", "HANDLER");

	// Recreate the author information with the hashed password
	let secure_author_info: AuthorForCreate  = AuthorForCreate {
		name: author_info.name,
		email: author_info.email,
		password
	};

	//  Create new author
	let author = app_state.create_author(secure_author_info).await.map_err(|e| Error::CouldNotCreateAuthor)?;

	// Create JWT
	let jwt = create_jwt(author.email.clone(), author.id)?;

	// Set auth header cookie
	cookies.add(Cookie::new(AUTHORIZATION_HEADER, format!("Bearer {}", jwt)));

	// Construct the author to be sent as a response
	let resulting_author = AuthorForResult {
		name: author.name,
		email: author.email
	};

	// Send successful sign up message
	let response = CustomResponse::<AuthorForResult>::new(
		true,
		Some(format!("Signed up successfully")),
		Some(CustomResponseData::Item(resulting_author))
	);

	Ok(Json(response))
}