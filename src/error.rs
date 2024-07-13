use crate::models;

pub type CoreResult<T> = core::result::Result<T, CoreError>;

#[derive(Debug)]
pub enum CoreError {
	// -- Config
	ConfigMissingEnv(&'static str),
	ConfigWrongFormat(&'static str),

	// -- Modules
	Model(models::ModelError)
}

// region:    --- Froms
impl From<models::ModelError> for CoreError {
	fn from(val: models::ModelError) -> Self {
		Self::Model(val)
	}
}
// endregion: --- Froms

// region:    --- Error Boilerplate
impl core::fmt::Display for CoreError {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for CoreError {}
// endregion: --- Error Boilerplate
