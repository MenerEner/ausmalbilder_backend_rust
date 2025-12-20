pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub password_hash: String,
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
        }
    }
}
