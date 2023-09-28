use axum::{Json, extract::State};
use chrono::format::format;
use tower_cookies::{Cookies, Cookie};

use crate::{models::{auth::LoginPayload, custom_response::{CustomResponse, CustomResponseData}, error::{Error, Result}, state::AppState, author::{AuthorForCreate, Author, AuthorForResult}}, middlewares::{AUTH_TOKEN, AUTHORIZATION_HEADER}, utils::auth::{create_jwt, hash_password, verify_hash}};

pub async fn handler_login(cookies: Cookies,  State(app_state): State<AppState>, Json(payload):  Json<LoginPayload>) -> Result<Json<CustomResponse<String>>>{
	println!("->> {:<12} - api_login", "HANDLER");
	let author_from_db = app_state.get_author_by_email(payload.email).await.map_err(|_| Error::InternalServerError)?;

	if let Ok(false) = verify_hash(payload.password, &author_from_db.password) {
		return Err(Error::LoginFail)
	}

	cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));
	let response = CustomResponse::<String>::new(
		true,
		Some(format!("Logged in Successfully")),
		None
	);

	Ok(Json(response))
}

pub async fn handler_signup(cookies: Cookies, State(app_state): State<AppState>, Json(author_info): Json<AuthorForCreate>) -> Result<Json<CustomResponse<AuthorForResult>>> {
	let password = hash_password(author_info.password.clone())?;
	println!("->> {:<12} - api_signup", "HANDLER");

	let secure_author_info: AuthorForCreate  = AuthorForCreate {
		name: author_info.name,
		email: author_info.email,
		password
	};

	let author = app_state.create_author(secure_author_info).await.map_err(|e| Error::CouldNotCreateAuthor)?;
	let jwt = create_jwt(author.email.clone())?;

	cookies.add(Cookie::new(AUTHORIZATION_HEADER, format!("Bearer {}", jwt)));

	let response = CustomResponse::<AuthorForResult>::new(
		true,
		Some(format!("Signed up successfully")),
		Some(CustomResponseData::Item(author))
	);

	Ok(Json(response))
}