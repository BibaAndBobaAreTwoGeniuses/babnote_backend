use std::collections::BTreeMap;

use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;

const SECRET_WORD: &str = "vladivostok"; // TODO: make it env

pub fn make_token(user_id: u32) -> String {
    let key: Hmac<Sha256> = Hmac::new_from_slice(SECRET_WORD.as_bytes()).unwrap();
    let mut claims: BTreeMap<String, u32> = BTreeMap::new();
    claims.insert("id".into(), user_id);
    
    let token_str = claims.sign_with_key(&key).unwrap();
    token_str
}

pub fn verify_token(token: &str) -> bool {
    todo!()
}