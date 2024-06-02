pub mod handlers;
pub mod middlewares;
pub mod routes;
pub mod error;
pub mod custom_response;
pub mod auth;


#[derive(Debug)]
pub struct HelloParams {
	name: Option<String>
}