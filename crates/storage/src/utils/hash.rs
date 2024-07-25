use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

#[allow(dead_code)]
pub fn verify_password(password: &str, hashed: &str) -> bool {
    let parsed_hash = PasswordHash::new(&hashed).expect("Failed to parse password hash");
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
}

#[allow(dead_code)]
pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let password = "anita.123";
        let hashed = hash_password(password);
        println!("hashed: {}", hashed);
        assert!(verify_password(password, &hashed));
    }
}
