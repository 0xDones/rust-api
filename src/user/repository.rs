use super::model::User;
use crate::infra::postgres::schema::users;
use crate::{diesel::prelude::*, infra::postgres::db::DbPool};

#[derive(Clone)]
pub struct UserRepository {
    pool: DbPool,
}

impl UserRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn create_user(&self, user: &User) -> Result<User, diesel::result::Error> {
        let conn = &self.pool.get().unwrap();

        diesel::insert_into(users::table)
            .values(user)
            .get_result::<User>(conn)
    }
}
