use std::io::{Error, ErrorKind};

use super::{dto::CreateUserDTO, model::User, repository::UserRepository};

#[derive(Clone)]
pub struct UserService {
    user_repo: UserRepository,
}

impl UserService {
    pub fn new(user_repo: UserRepository) -> Self {
        Self { user_repo }
    }

    pub fn get_user(&self, user_id: i32) -> Result<User, Error> {
        println!("{}", format!("Getting user with id {}", user_id));
        match self.user_repo.get_user(user_id) {
            Ok(user) => Ok(user),
            Err(e) => {
                // Return domain error
                println!("Failed to create user: {}", e);
                Err(Error::new(ErrorKind::NotFound, "User not found"))
            }
        }
    }

    pub fn create_user(&self, create_user_dto: &CreateUserDTO) -> Result<User, Error> {
        println!("Creating user...");
        let user = User::new();

        match self.user_repo.create_user(&user) {
            Ok(user) => {
                println!("Created mf user");
                Ok(user)
            }
            Err(e) => {
                // Return domain error
                println!("Failed to create user: {}", e);
                Err(Error::new(ErrorKind::Other, "User already exist"))
            }
        }
    }
}
