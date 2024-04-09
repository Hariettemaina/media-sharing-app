use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, Error, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};


pub struct PassWordHasher<'a> {
    pub argon2: Argon2<'a>,
    pub salt: SaltString,
}

impl PassWordHasher<'_> {
    pub fn new() -> Self {
        Self {
            argon2: Argon2::default(),
            salt: SaltString::generate(&mut OsRng),
        }
    }
    pub fn hash_password(&self, password: String) -> Result<String, Error> {
        let password_hash = self
            .argon2
            .hash_password(password.as_bytes(), &self.salt)?
            .to_string();

        Ok(password_hash)
    }

    pub fn verify_password(&self, password: String, password_hash: String) -> bool {
        let Ok(hash) = PasswordHash::new(&password_hash) else { return false };
        self.argon2
            .verify_password(password.as_bytes(), &hash)
            .is_ok()
    }
}
