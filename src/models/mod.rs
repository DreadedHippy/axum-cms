use serde::Deserialize;

pub mod author;
pub mod error;
pub mod custom_response;

#[derive(Debug, Deserialize)]
pub struct HelloParams {
	pub name: Option<String>
}