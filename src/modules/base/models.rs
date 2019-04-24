extern crate dotenv;
extern crate argon2;
extern crate chrono;

use dotenv::dotenv;
use std::env;
// use chrono::{Utc, DateTime};
use libs::models::BaseModel;

#[derive(Debug)]
pub struct User {
    base: BaseModel,
    username: String,
    email: String,
    password: String,
}

impl User {
    pub fn new(id: i32, username: String, email: String, password: String) -> User {
        User {
            base: BaseModel::new(id),
            username,
            email,
            password: User::hashing_password(&password),
        }
    }

    fn hashing_password(password: &String) -> String {
        dotenv().ok();

        let config = argon2::Config::default();
        let secret = env::var("SECRET").expect("'Secret Key' must be set!");
        argon2::hash_encoded(password.as_bytes(), secret.as_bytes(), &config).unwrap()
    }
}
