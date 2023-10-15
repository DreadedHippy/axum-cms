use serde::Deserialize;
/// All models pertaining to authors
pub mod author;
/// All models pertaining to errors encountered during route handling
pub mod error;
/// All models pertaining to custom JSON responses
pub mod custom_response;
/// All models pertaining to posts
pub mod post;
/// All models pertaining to the App state
pub mod state;
/// All models pertaining to authentication
pub mod auth;
#[derive(Debug, Deserialize)]
pub struct HelloParams {
	pub name: Option<String>
}