use super::{CryptError, CryptResult};
use crate::config;
use crate::crypt::{encrypt_into_b64url, EncryptContent};

/// Encrypt password with default scheme.
pub fn encrypt_pwd(encrypt_content: &EncryptContent) -> CryptResult<String> {
	let key = &config().PWD_KEY;

	let encrypted = encrypt_into_b64url(key, encrypt_content)?;

	Ok(format!("#01#{encrypted}"))
}

/// Validate if encrypt content matches.
pub fn validate_pwd(encrypt_content: &EncryptContent, pwd_ref: &str) -> CryptResult<()> {
	let pwd = encrypt_pwd(encrypt_content)?;

	if pwd == pwd_ref {
		Ok(())
	} else {
		Err(CryptError::PwdNotMatching)
	}


}