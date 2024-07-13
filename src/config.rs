use axum::Server;

// use crate::{Result, models::{error::{Error, Result}, self}};
use std::{env, str::FromStr, sync::OnceLock};

use crate::error::{CoreError, CoreResult};


pub fn config() -> &'static Config {
	static INSTANCE: OnceLock<Config> = OnceLock::new();

	INSTANCE.get_or_init(|| {
		Config::load_from_env().unwrap_or_else(|ex| {
			panic!("FATAL - WHILE LOADING CONF -Cause: {ex:?}")
		})
	})
}



#[allow(non_snake_case)]
pub struct Config {
	// -- Crypt
	pub PWD_KEY: Vec<u8>,

	pub TOKEN_KEY: Vec<u8>,
	pub TOKEN_DURATION_SEC: f64,
	// -- Db
	pub DB_URL: String,
	// -- Web
	pub WEB_FOLDER: String
}

impl Config {
	fn load_from_env() -> CoreResult<Config> {
		Ok(Config {
			// -- Crypt
			PWD_KEY: get_env_b64url_as_u8s("SERVICE_PWD_KEY")?,
		
			TOKEN_KEY: get_env_b64url_as_u8s("SERVICE_TOKEN_KEY")?,
			TOKEN_DURATION_SEC: get_env_parse("SERVICE_TOKEN_DURATION_SEC")?,
			// -- Db
			DB_URL: get_env("SERVICE_DB_URL")?,
			// -- Web
			WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?
		})
	}
}

fn get_env(name: &'static str) -> CoreResult<String> {
	env::var(name).map_err(|_| CoreError::ConfigMissingEnv(name))
}

fn get_env_parse<T: FromStr>(name: &'static str) -> CoreResult<T> {
	let val = get_env(name)?;
	val.parse::<T>().map_err(|_| CoreError::ConfigWrongFormat(name))
}

fn get_env_b64url_as_u8s(name: &'static str) -> CoreResult<Vec<u8>> {
	base64_url::decode(&get_env(name)?).map_err(|_| CoreError::ConfigWrongFormat(name))
}
