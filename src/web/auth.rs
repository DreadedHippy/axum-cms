use serde::{Deserialize, Serialize};

use crate::models::author::AuthorForCreate;

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
	pub email: String,
	pub password: String
}

#[derive(Debug, Deserialize)]
pub struct SignupPayload {
	pub name: String,
	pub email: String,
	pub password: String
}

impl From<SignupPayload> for AuthorForCreate {
	fn from(value: SignupPayload) -> Self {
		AuthorForCreate {
			name: value.name,
			email: value.email,
			password: value.password
		}
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims { // For JWT creation
	pub email: String,
	pub exp: usize,
	pub iat: usize,
	pub id: i64
}


