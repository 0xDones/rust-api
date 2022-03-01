use std::sync::Arc;

use super::model::User;
use crate::infra::postgres::schema::users;
use crate::{diesel::prelude::*, infra::postgres::db::DbPool};

#[derive(Clone)]
pub struct UserRepository {
    pool: Arc<DbPool>,
}

impl UserRepository {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self { pool }
    }

    fn get_conn(self) -> PgConnection {
        let mut conn = self.pool.get().unwrap();
        *conn
    }

    pub fn create_user(self, user: User) -> Result<User, diesel::result::Error> {
        let conn = self.get_conn();

        diesel::insert_into(users::table)
            .values(&user)
            .get_result::<User>(&conn)
    }
}
