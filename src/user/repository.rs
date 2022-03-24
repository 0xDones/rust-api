use super::model::User;
use crate::diesel::prelude::*;
use crate::infra::postgres::{db::DbPool, schema::users};

#[derive(Clone)]
pub struct UserRepository {
    pool: DbPool,
}

impl UserRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn get_user(&self, user_id: i32) -> Result<User, diesel::result::Error> {
        let conn = &self.pool.get().unwrap();

        users::table.find(user_id).first(conn)
    }

    pub fn create_user(&self, user: &User) -> Result<User, diesel::result::Error> {
        let conn = &self.pool.get().unwrap();

        diesel::insert_into(users::table)
            .values(user)
            .get_result::<User>(conn)
    }
}
