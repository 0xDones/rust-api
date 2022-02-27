use std::io::Error;

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

    fn get_conn(self) -> PgConnection {
        let mut conn = self.pool.get().unwrap();
        *conn
    }

    pub fn create_user(self, user: User) -> Result<(), Error> {
        let conn = self.get_conn();

        diesel::insert_into(users::table)
            .values(&user)
            .get_result::<User>(&conn)
            .expect("Error saving new post");
        Ok(())
    }
}
