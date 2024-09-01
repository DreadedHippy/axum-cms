use serde::Serialize;

pub type StoreResult<T> = core::result::Result<T, StoreError>;

#[derive(Debug, Serialize)] // This is very flexible for logging into t a new-line JSON format
pub enum StoreError{
	FailToCreatePool(String)
}

// region: --Error Boilerplate
impl core::fmt::Display for StoreError {
	fn fmt(
		&self,
		fmt: &mut std::fmt::Formatter
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for StoreError{}
// endregion: --Error Boilerplate