pub type UtilResult<T> = Result<T, UtilError>;

#[derive(Debug)]
pub enum UtilError {
	// Time
	DateFailParse(String),

	// Base64
	FailToB64uDecode
}

impl core::fmt::Display for UtilError {
	fn fmt(
		&self,
		fmt: &mut std::fmt::Formatter
	) -> std::fmt::Result {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for UtilError {}
