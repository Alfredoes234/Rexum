use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use anyhow::{Result, Error};

pub fn hash_password(pwd: String) -> Result<String, argon2::password_hash::Error> {
    let password = pwd;
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();
    Ok(password_hash)
}

pub fn verify_password(pwd: String, hashed: &str) -> Result<bool, Error>  {
    let argon2 = Argon2::default();
    let verify = argon2.verify_password(pwd.as_bytes(), &PasswordHash::new(hashed).unwrap());
    if verify.is_ok() {
        Ok(true)
    } else {
        Ok(false)
    }
}