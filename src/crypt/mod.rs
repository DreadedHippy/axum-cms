// region:    --- Modules

mod error;
pub mod pwd;
pub mod token;

pub use self::error::{CryptError, CryptResult};

use hmac::{Hmac, Mac};
use sha2::Sha512;
// endregion: --- Modules

pub struct EncryptContent {
	pub content: String, // Clear content
	pub salt: String,    // Clear salt
}

pub fn encrypt_into_b64url(
	key: &[u8],
	encrypt_content: &EncryptContent
) -> CryptResult<String> {
	let EncryptContent {content, salt} = encrypt_content;

	// -- Create a HMAC-SHA-512 from key.
	let mut hmac_sha_512 = 
		Hmac::<Sha512>::new_from_slice(key).map_err(|_| CryptError::KeyFailHmac)?;
	
	// -- Add content.
	hmac_sha_512.update(content.as_bytes());
	hmac_sha_512.update(salt.as_bytes());

	// -- Finalize and b64u encode.
	let hmac_result =  hmac_sha_512.finalize();
	let result_bytes = hmac_result.into_bytes();

	let result = base64_url::encode(&result_bytes);

	Ok(result)

}

#[cfg(test)]
mod tests {
	use super::*;
	use anyhow::Result;
	use rand::RngCore;

	#[test]
	fn test_encrypt_into_b64url_ok() -> Result<()> {
		// -- Setup & Fixture
		let mut fx_key = [0u8; 64]; // 512 bits = 64 bytes;
		rand::thread_rng().fill_bytes(&mut fx_key);

		let fx_enc_content = EncryptContent {
			content: "Hello World".to_string(),
			salt: "some pepper".to_string()
		};

		// TODO: Need to fix fx_key and precompute fx_res.
		let fx_res = encrypt_into_b64url(&fx_key, &fx_enc_content)?;

		// -- Exec
		let res = encrypt_into_b64url(&fx_key, &fx_enc_content)?;

		// -- Check
		assert_eq!(res, fx_res);


		Ok(())
	}
}