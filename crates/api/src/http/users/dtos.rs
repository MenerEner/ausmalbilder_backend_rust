use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 1))]
    pub first_name: String,
    #[validate(length(min = 1))]
    pub last_name: String,
    #[validate(email)]
    pub email: String,
    pub phone_number: Option<String>,
    #[validate(length(min = 8))]
    pub password: String,
    pub birth_date: Option<NaiveDate>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub is_email_verified: bool,
    pub role: String,
}

impl From<domain_users::User> for UserResponse {
    fn from(user: domain_users::User) -> Self {
        Self {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            phone_number: user.phone_number,
            birth_date: user.birth_date,
            is_email_verified: user.is_email_verified,
            role: user.role.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct VerifyEmailRequest {
    pub token: String,
}

#[derive(Debug, Deserialize, ToSchema, IntoParams, Validate)]
pub struct PaginationParams {
    #[validate(range(min = 0))]
    pub page: Option<u64>,
    #[validate(range(min = 1, max = 100))]
    pub page_size: Option<u64>,
}

impl PaginationParams {
    pub fn page(&self) -> u64 {
        self.page.unwrap_or(0)
    }

    pub fn page_size(&self) -> u64 {
        self.page_size.unwrap_or(20)
    }
}
