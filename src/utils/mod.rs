use std::env;

use axum::http::Method;
use axum::http::Uri;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use serde_json::json;
use sqlx::postgres::PgConnectOptions;
use sqlx::{PgPool, Pool, Postgres};
use tracing::debug;
use uuid::Uuid;

use crate::ctx::Ctx;
use crate::web::ServerError;
use crate::log::log_request;

// pub mod auth;
// pub mod cache;
pub mod error;

pub use self::error::{UtilError, UtilResult};
use time::format_description::well_known::Rfc3339;
use time::{Duration, OffsetDateTime};

// region:    --- Time
pub fn now_utc() -> OffsetDateTime {
	OffsetDateTime::now_utc()
}

pub fn format_time(time: OffsetDateTime) -> String {
	time.format(&Rfc3339).unwrap() // TODO: Need to check if safe
}

pub fn now_utc_plus_sec_str(sec: f64) -> String {
	let new_time = now_utc() + Duration::seconds_f64(sec);
	format_time(new_time)
}

pub fn parse_utc(moment: &str) -> UtilResult<OffsetDateTime> {
	OffsetDateTime::parse(moment, &Rfc3339).map_err(|_| {
		UtilError::DateFailParse(moment.to_string())
	})
}
// endregion: --- Time


// region:    --- Base64
pub fn b64u_encode(content: &str) -> String {
	base64_url::encode(content)
}

pub fn b64u_decode(b64u: &str) -> UtilResult<String> {
	let decoded_str = base64_url::decode(b64u)
	.ok()
	.and_then(|r| String::from_utf8(r).ok())
	.ok_or(UtilError::FailToB64uDecode)?;

	Ok(decoded_str)
}
// endregion: --- Base64
const JWT_DURATION_IN_SECONDS: i64 = 60 * 60 * 2;