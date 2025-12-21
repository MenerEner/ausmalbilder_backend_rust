use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub password_hash: String,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(
        id: uuid::Uuid,
        name: String,
        email: String,
        phone_number: Option<String>,
        password_hash: String,
    ) -> Self {
        Self {
            id,
            name,
            email,
            phone_number,
            password_hash,
            deleted_at: None,
        }
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
