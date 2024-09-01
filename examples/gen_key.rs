use anyhow::Result;
use rand::RngCore;

fn main() -> Result<()> {
	let mut key = [0u8; 64]; // 512 bits = 64 bytes;
	rand::thread_rng().fill_bytes(&mut key);
	println!("\nGenerated key for HMC: \n{key:?}");

	let b64url = base64_url::encode(&key);
	println!("\nKey b64url encoded:\n{b64url}");

	Ok(())

}