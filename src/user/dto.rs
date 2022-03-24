use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CreateUserDTO {
    pub username: String,
    pub email: String,
}
