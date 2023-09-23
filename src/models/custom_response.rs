use serde::Serialize;

#[derive(Serialize)]
pub struct CustomResponse<T> {
	pub status: bool,
	pub message: Option<String>,
	pub data: Option<CustomResponseData<T>>
}

impl<T> CustomResponse<T> {
	pub fn new(status: bool, message: Option<String>, data: Option<CustomResponseData<T>>) -> Self {
		Self { status, message, data }
	}
}


#[derive(Serialize)]
#[serde(untagged)]
pub enum CustomResponseData<T> {
	Text(String),
	Collection(Vec<T>),
	Item(T)
}