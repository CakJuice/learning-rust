extern crate chrono;

use chrono::{Utc, DateTime};

#[derive(Debug)]
pub struct BaseModel {
    id: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl BaseModel {
    pub fn new(id: i32) -> BaseModel {
        BaseModel {
            id: id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}