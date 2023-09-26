use serde::{Deserialize, Serialize};

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

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
	pub email: String,
	pub exp: usize,
	pub iat: usize
}


