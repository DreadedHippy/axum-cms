use std::fmt::Display;

use serde::Serialize;

#[derive(Serialize, Debug)]
/// Custom response struct to hold the JSON fields sent back to the client as a response
pub struct CustomResponse<T> {
	pub status: bool,
	pub message: Option<String>,
	pub data: Option<CustomResponseData<T>>
}

impl<T> CustomResponse<T>{
	/// Generate a custom response.
	pub fn new(status: bool, message: Option<String>, data: Option<CustomResponseData<T>>) -> Self {
		Self { status, message, data }
	}
}

impl<T> core::fmt::Display for CustomResponse<T> where T: std::fmt::Debug {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}


#[derive(Serialize, Debug)]
#[serde(untagged)]
/// Enum built with a generic holding the data sent in the `data` field of `CustomResponse`
pub enum CustomResponseData<T> {
	Text(String),
	Collection(Vec<T>),
	Item(T)
}