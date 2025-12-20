use sea_orm::DatabaseConnection;
use std::sync::Arc;
use application::use_cases::CreateUserUseCase;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub create_user_use_case: Arc<CreateUserUseCase>,
}

impl AppState {
    pub fn new(db: DatabaseConnection, create_user_use_case: CreateUserUseCase) -> Self {
        Self {
            db,
            create_user_use_case: Arc::new(create_user_use_case),
        }
    }
}
