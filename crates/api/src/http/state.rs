use application::use_cases::{CreateUserUseCase, GetUserUseCase};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub create_user_use_case: Arc<CreateUserUseCase>,
    pub get_user_use_case: Arc<GetUserUseCase>,
}

impl AppState {
    pub fn new(
        db: DatabaseConnection,
        create_user_use_case: CreateUserUseCase,
        get_user_use_case: GetUserUseCase,
    ) -> Self {
        Self {
            db,
            create_user_use_case: Arc::new(create_user_use_case),
            get_user_use_case: Arc::new(get_user_use_case),
        }
    }
}
