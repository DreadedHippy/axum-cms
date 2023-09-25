use axum::{response::IntoResponse, http::StatusCode};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	LoginFail,

	// -- Auth errors.
	AuthFailNoAuthTokenCookie,

	// -- Model errors.
	
	// Author
	CouldNotCreateAuthor,
	CouldNotGetAuthors,
	CouldNotGetAuthor,

	// Post
	CouldNotCreatePost,
	CouldNotGetPosts,
	CouldNotGetPost

	// -- 
}

impl IntoResponse for Error {
	fn into_response(self) -> axum::response::Response {
		println!("->> {:<12} - {self:?}", "INTO_RES");

		(StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
	}
}