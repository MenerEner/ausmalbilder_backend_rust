use application::use_cases::{CreateUserUseCase, DeleteUserUseCase, GetUserUseCase, ListUsersUseCase};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub create_user_use_case: Arc<CreateUserUseCase>,
    pub get_user_use_case: Arc<GetUserUseCase>,
    pub delete_user_use_case: Arc<DeleteUserUseCase>,
    pub list_users_use_case: Arc<ListUsersUseCase>,
}

impl AppState {
    pub fn new(
        db: DatabaseConnection,
        create_user_use_case: CreateUserUseCase,
        get_user_use_case: GetUserUseCase,
        delete_user_use_case: DeleteUserUseCase,
        list_users_use_case: ListUsersUseCase,
    ) -> Self {
        Self {
            db,
            create_user_use_case: Arc::new(create_user_use_case),
            get_user_use_case: Arc::new(get_user_use_case),
            delete_user_use_case: Arc::new(delete_user_use_case),
            list_users_use_case: Arc::new(list_users_use_case),
        }
    }
}
