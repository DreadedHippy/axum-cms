use std::fmt::Display;
use std::str::FromStr;

use axum::body::HttpBody;

use crate::config;
use crate::crypt::{CryptError, CryptResult};
use crate::utils::{b64u_decode, b64u_encode, now_utc, now_utc_plus_sec_str, parse_utc};

use super::{encrypt_into_b64url, EncryptContent};

// region:    --- TokenType

/// String format: `ident_b64u.exp_b64u.sign_b64u`
#[derive(Debug)]
pub struct Token {
	pub ident: String, // identifier (Email for example)
	pub exp: String, // Expiration date in Rfc3339
	pub sign_b64u: String, // Signature, base64url encoded.
}

impl FromStr for Token {
	type Err = CryptError;

	fn from_str(token_str: &str) -> Result<Self, Self::Err> {
		let splits: Vec<&str> = token_str.split(".").collect();

		if splits.len() != 3 {
			return Err(CryptError::TokenInvalidFormat)
		}

		let (ident_b64u, exp_b64u, sign_b64u) = (splits[0], splits[1], splits[2]);

		Ok(Self {
			ident: b64u_decode(ident_b64u).map_err(|_| CryptError::TokenCannotDecodeIdent)?,
			exp: b64u_decode(exp_b64u).map_err(|_| CryptError::TokenCannotDecodeExp)?,
			sign_b64u: sign_b64u.to_string()
		})
			
	}
}

impl Display for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}.{}.{}",
			b64u_encode(&self.ident),
			b64u_encode(&self.exp),
			self.sign_b64u
		)
	}
}

// endregion: --- TokenType

// region:    --- Web token Gen and Validation

pub fn generate_web_token(user: &str, salt: &str) -> CryptResult<Token> {
	let config = &config();

	_generate_token(user, config.TOKEN_DURATION_SEC, salt, &config.TOKEN_KEY)
}

pub fn validate_web_token(origin_token: &Token, salt: &str) -> CryptResult<()>{
	let config = &config();

	_validate_token_sign_and_exp(origin_token, salt, &config.TOKEN_KEY)?;

	Ok(())
}
// endregion: --- Web token Gen and Validation

// region:    --- (private) Token generation an validation

fn _generate_token(
	ident: &str,
	duration_sec: f64,
	salt: &str,
	key: &[u8]
) -> CryptResult<Token>{
	// -- Compute the two first components
	let ident = ident.to_string();

	let exp = now_utc_plus_sec_str(duration_sec);

	// -- Sign the first two components
	let sign_b64u = _token_sign_into_b64u(&ident, &exp, salt, key)?;

	Ok(Token {ident, exp, sign_b64u})
}

fn _validate_token_sign_and_exp(
	origin_token: &Token,
	salt: &str,
	key: &[u8]
) -> CryptResult<()> {
	// -- Validate signature
	let new_sign_b64u = _token_sign_into_b64u(&origin_token.ident, &origin_token.exp, salt, key)?;

	if new_sign_b64u != origin_token.sign_b64u {
		return Err(CryptError::TokenSignatureNotMatching)
	}

	let origin_exp = parse_utc(&origin_token.exp).map_err(|_| CryptError::TokenExpNotIso)?;

	let now = now_utc();

	if origin_exp < now {
		return Err(CryptError::TokenExpired)
	}

	Ok(())
}

/// Create token signature from token parts
/// and salt.
fn _token_sign_into_b64u(
	ident: &str,
	exp: &str,
	salt: &str,
	key: &[u8]
) -> CryptResult<String> {
	let content = format!("{}.{}", b64u_encode(ident), b64u_encode(exp));
	let signature = encrypt_into_b64url(
		key,

		&EncryptContent {
			content,
			salt: salt.to_string()
		}
	)?;

	Ok(signature)
}

// endregion: --- (private) Token generation an validation


// region:    --- Tests
#[cfg(test)]
mod tests {
	use std::{thread, time::Duration};

	use super::*;
	use anyhow::Result;

	#[test]
	fn test_token_display_ok() -> Result<()> {
		let fx_token = Token {
			ident: "fx-ident-01".to_string(),
			exp: "2023-05-17T15:30:00Z".to_string(),
			sign_b64u: "some-sign-b64u-encoded".to_string(),
		};

		let fx_token_str = "ZngtaWRlbnQtMDE.MjAyMy0wNS0xN1QxNTozMDowMFo.some-sign-b64u-encoded";

		// -- Exec
		let token: Token = fx_token_str.parse()?;

		// -- Check
		assert_eq!(format!("{token:?}"), format!("{fx_token:?}"));
		

		Ok(())
	}

	#[test]
	fn test_validate_web_token_ok() -> Result<()> {
		// -- Setup & fixtures
		let fx_user = "user_one";
		let fx_salt = "pepper";
		let fx_duration_sec = 0.02; //20ms
		let token_key = &config().TOKEN_KEY;

		let fx_token = _generate_token(fx_user, fx_duration_sec, fx_salt, token_key)?;

		// -- Exec
		thread::sleep(Duration::from_millis(10));

		// -- Check
		let res = validate_web_token(&fx_token, &fx_salt);

		res?;

		Ok(())
	}

	
	#[test]
	fn test_validate_web_token_err_expired() -> Result<()> {
		// -- Setup & fixtures
		let fx_user = "user_one";
		let fx_salt = "pepper";
		let fx_duration_sec = 0.01; // 10ms
		let token_key = &config().TOKEN_KEY;

		let fx_token = _generate_token(fx_user, fx_duration_sec, fx_salt, token_key)?;

		// -- Exec
		thread::sleep(Duration::from_millis(20)); // sleep for 20 ms, 10ms more than token duration
		let res = validate_web_token(&fx_token, &fx_salt);

		assert!(
			matches!(res, Err(CryptError::TokenExpired)),
			"Should have matched `Err(CryptError::TokenExpired)` but was `{res:?}`"
		);

		Ok(())
	}
	
}
// endregion: --- Tests
