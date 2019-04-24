extern crate chrono;

use chrono::{Utc, DateTime};

#[warn(dead_code)]
struct BaseModel {
    id: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl BaseModel {
    fn new(id: i32) -> BaseModel {
        BaseModel {
            id: id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}