use serde::Serialize;

pub type CryptResult<T> = core::result::Result<T, CryptError>;

#[derive(Debug, Serialize)]
pub enum CryptError {
	// Key
	KeyFailHmac,

	// Pwd
	PwdNotMatching,

	// Token
	TokenInvalidFormat,
	TokenCannotDecodeIdent,
	TokenCannotDecodeExp,
	TokenSignatureNotMatching,
	TokenExpNotIso,
	TokenExpired
}

// region:    --- Error Boilerplate
impl core::fmt::Display for CryptError {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for CryptError{}
// endregion: --- Error Boilerplate