use bcrypt::DEFAULT_COST;


pub fn encrypt_password(password: &str) -> String {
    bcrypt::hash(password, DEFAULT_COST).unwrap()
}