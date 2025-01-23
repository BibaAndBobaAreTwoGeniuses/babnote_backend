use bcrypt::{verify, DEFAULT_COST};


pub fn encrypt_password(password: &str) -> String {
    bcrypt::hash(password, DEFAULT_COST).unwrap()
}
pub fn validate_password(password: &str, hashed_password: &str) -> bool {
    verify(password, hashed_password).unwrap()
}