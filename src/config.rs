// use crate::{Result, models::{error::{Error, Result}, self}};
use crate::models::error::{ServerResult as CustomResult, ServerError as CustomError};
use std::{env, sync::OnceLock};

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
	// -- Web
	pub WEB_FOLDER: String

}

impl Config {
	fn load_from_env() -> CustomResult<Config> {
		Ok(Config {
			// -- Web
		WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?
		})
	}
}

// fn get_env(name: &'static str) -> crate::Result<String> {
// 	env::var(name).map_err(|_| models::error::Error::ConfigMissingEnv(name))
// }

fn get_env(name: &'static str) -> CustomResult<String> {
		env::var(name).map_err(|_| CustomError::ConfigMissingEnv(name))
	}