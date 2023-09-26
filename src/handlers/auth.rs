use axum::{Json, extract::State};
use chrono::format::format;
use tower_cookies::{Cookies, Cookie};

use crate::{models::{auth::LoginPayload, custom_response::{CustomResponse, CustomResponseData}, error::{Error, Result}, state::AppState, author::{AuthorForCreate, Author}}, middlewares::{AUTH_TOKEN, AUTHORIZATION_HEADER}, utils::auth::create_jwt};

pub async fn handler_login(cookies: Cookies, Json(payload):  Json<LoginPayload>) -> Result<Json<CustomResponse<String>>>{
	println!("->> {:<12} - api_login", "HANDLER");

	if payload.email != "email@mail.com" || payload.password != "password" {
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

pub async fn handler_signup(cookies: Cookies, State(app_state): State<AppState>, Json(author_info): Json<AuthorForCreate>) -> Result<Json<CustomResponse<Author>>> {
	println!("->> {:<12} - api_signup", "HANDLER");

	let author = app_state.create_author(author_info).await.map_err(|e| Error::CouldNotCreateAuthor)?;
	let jwt = create_jwt(author.email.clone())?;

	cookies.add(Cookie::new(AUTHORIZATION_HEADER, format!("Bearer {}", jwt)));

	let response = CustomResponse::<Author>::new(
		true,
		Some(format!("Signed up successfully")),
		Some(CustomResponseData::Item(author))
	);

	Ok(Json(response))
}