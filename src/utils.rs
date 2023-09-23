use axum::response::Response;

pub async fn main_response_mapper(res:Response) -> Response {
	println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

	println!();
	res
}