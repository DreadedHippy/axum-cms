use serde::Serialize;

pub type CtxResult<T> = core::result::Result<T, CtxError>;

#[derive(Debug, Serialize)]
pub enum CtxError {
	CtxCannotNewRootCtx,
}

// region:    --- Error Boilerplate
impl core::fmt::Display for CtxError {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for CtxError {}
// endregion: --- Error Boilerplate