use dotenv::dotenv;
use std::env;
// use chrono::{Utc, DateTime};

#[derive(Debug)]
enum UserType {
    superuser,
    officer,
    manager,
}

#[derive(Debug)]
pub struct User {
    // base: BaseModel,
    id: i32,
    username: String,
    email: String,
    password: String,
    user_type: UserType,
}

impl User {
    pub fn new(id: i32, username: String, email: String, password: String) -> User {
        User {
            // base: BaseModel::new(id),
            id,
            username,
            email,
            password: User::hashing_password(&password),
            user_type: UserType::superuser,
        }
    }

    fn hashing_password(password: &String) -> String {
        dotenv().ok();

        let config = argon2::Config::default();
        let secret = env::var("SECRET").expect("'Secret Key' must be set!");
        argon2::hash_encoded(password.as_bytes(), secret.as_bytes(), &config).unwrap()
    }
}
