use wasm_bindgen::prelude::*;
use rsa::RsaPrivateKey;
use pkcs8::{EncodePrivateKey, LineEnding};

#[wasm_bindgen]
pub fn generate_key() -> String {
    let mut rng = rand::thread_rng();
    let bits = 4096;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    return EncodePrivateKey::to_pkcs8_pem(&priv_key, LineEnding::LF).unwrap().to_string();
}