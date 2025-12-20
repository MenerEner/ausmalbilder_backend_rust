use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub phone_number: Option<String>,
}

impl From<domain_users::User> for UserResponse {
    fn from(user: domain_users::User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            phone_number: user.phone_number,
        }
    }
}
