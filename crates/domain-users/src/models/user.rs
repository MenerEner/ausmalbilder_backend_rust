use chrono::{DateTime, NaiveDate, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserRole {
    User,
    VerifiedUser,
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::User => write!(f, "user"),
            UserRole::VerifiedUser => write!(f, "verified_user"),
        }
    }
}

impl From<String> for UserRole {
    fn from(s: String) -> Self {
        match s.as_str() {
            "verified_user" => UserRole::VerifiedUser,
            _ => UserRole::User,
        }
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub password_hash: String,
    pub birth_date: Option<NaiveDate>,
    pub is_email_verified: bool,
    pub role: UserRole,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(
        id: uuid::Uuid,
        first_name: String,
        last_name: String,
        email: String,
        phone_number: Option<String>,
        password_hash: String,
        birth_date: Option<NaiveDate>,
    ) -> Self {
        Self {
            id,
            first_name,
            last_name,
            email,
            phone_number,
            password_hash,
            birth_date,
            is_email_verified: false,
            role: UserRole::User,
            deleted_at: None,
        }
    }

    pub fn verify_email(&mut self) {
        self.is_email_verified = true;
        self.role = UserRole::VerifiedUser;
    }

    pub fn delete(&mut self) {
        if self.deleted_at.is_none() {
            self.deleted_at = Some(Utc::now());
        }
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }
}
