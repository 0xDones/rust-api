use std::{io::Error, thread, time};

use super::{model::User, repository::UserRepository};

#[derive(Clone)]
pub struct UserService {
    user_repo: UserRepository,
}

impl UserService {
    pub fn new(user_repo: UserRepository) -> Self {
        Self { user_repo }
    }

    pub fn get_user(self, id: &str) -> Result<User, Error> {
        println!("{}", format!("Getting user with id {}", id));
        let ten_millis = time::Duration::from_millis(2000);

        thread::sleep(ten_millis);
        println!("{}", format!("Returning user with id {}", id));
        let user = User::new();
        Ok(user)
    }

    pub fn create_user(self) -> Result<User, Error> {
        println!("Creating user...");
        let user = User::new();
        self.user_repo.create_user(user);
        println!("User created.");
        Ok(user)
    }
}
