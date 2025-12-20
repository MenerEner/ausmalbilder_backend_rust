use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
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
