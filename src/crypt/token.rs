use std::collections::BTreeMap;

use hmac::{Hmac, Mac};
use jwt::{Header, SignWithKey, Token, VerifyWithKey};
use sha2::Sha256;

use crate::managers;

const SECRET_WORD: &str = "vladivostok"; // TODO: make it env

pub fn make_token(user_id: i32) -> String {
    let key: Hmac<Sha256> = Hmac::new_from_slice(SECRET_WORD.as_bytes()).unwrap();
    let mut claims: BTreeMap<String, i32> = BTreeMap::new();
    claims.insert("id".into(), user_id);
    
    let token_str = claims.sign_with_key(&key).unwrap();
    token_str
}

pub fn verify_token(token: &str) -> anyhow::Result<u32> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(SECRET_WORD.as_bytes()).unwrap();
    let claims: BTreeMap<String, u32> = match token.verify_with_key(&key) {
        Ok(claims) => claims,
        Err(why) => {
            eprintln!("{}", why);
            return Err(why.into())
        }
    };
    Ok(claims["id"])
}