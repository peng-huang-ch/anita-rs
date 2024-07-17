use bcrypt::{hash, verify, DEFAULT_COST};

pub fn verify_password(password: &str, hash: &str) -> bool {
    verify(password, hash).unwrap_or(false)
}

pub fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).expect("Failed to hash password")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let password = "anita.123";
        let hashed = hash(password, DEFAULT_COST).expect("Failed to hash password");
        assert!(verify_password(password, &hashed));
    }
}
