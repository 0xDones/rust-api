use crate::infra::postgres::schema::users;
use std::time::SystemTime;

#[derive(Queryable, Insertable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: Option<SystemTime>,
}

impl User {
    pub fn new() -> Self {
        Self {
            id: 123,
            username: "username".to_string(),
            email: "email".to_string(),
            created_at: Some(SystemTime::now()),
        }
    }
}
