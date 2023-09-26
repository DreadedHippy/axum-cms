use serde::Deserialize;

pub mod author;
pub mod error;
pub mod custom_response;
pub mod post;
pub mod state;
pub mod auth;

#[derive(Debug, Deserialize)]
pub struct HelloParams {
	pub name: Option<String>
}