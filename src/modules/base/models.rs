extern crate dotenv;
extern crate argon2;
extern crate chrono;

use dotenv::dotenv;
use std::env;
use chrono::{Utc, DateTime};

#[derive(Debug)]
pub struct User {
    username: String,
    email: String,
    password: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(username: String, email: String, password: String) -> User {
        User {
            username,
            email,
            password: User::hashing_password(&password),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn hashing_password(password: &String) -> String {
        dotenv().ok();

        let config = argon2::Config::default();
        let secret = env::var("SECRET").unwrap();
        argon2::hash_encoded(password.as_bytes(), secret.as_bytes(), &config).unwrap()
    }
}
