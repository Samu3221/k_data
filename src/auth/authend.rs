use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use rand_core::OsRng;
use std::fs;

use rsa::{
    RsaPrivateKey,
    pkcs1::DecodeRsaPrivateKey,
    pss::BlindedSigningKey,
    sha2::Sha256,
    signature::{RandomizedSigner, SignatureEncoding},
};

// currently spaghetti code it just needs to work
pub mod authentication {

    use super::*;

    pub(crate) fn sign(secret_key_path: &str, timestamp_ms: u64, method: &str, path: &str) -> String {
        // getting the secret key from the file
        let secret_key_pem = match fs::read_to_string(secret_key_path) {
            Ok(content) => content,
            Err(_) => panic!("Program failed to retrieve secret key in pem format"),
        };

        // code modified from https://github.com/pbeets/kalshi-trade-rs/blob/main/src/auth.rs#L106
        let private_key = match RsaPrivateKey::from_pkcs1_pem(&secret_key_pem) {
            Ok(private_key) => private_key,
            Err(error) => panic!("Program failed to encode the private_key,{}", error),
        };
        // Message format: {timestamp_ms}{METHOD}{path}
        let message = format!("{}{}{}", timestamp_ms, method.to_uppercase(), path);

        let signing_key = BlindedSigningKey::<Sha256>::new(private_key);
        let signature = signing_key.sign_with_rng(&mut OsRng, message.as_bytes());

        BASE64.encode(signature.to_bytes())
    }
}
