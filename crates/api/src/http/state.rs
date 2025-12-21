use application::use_cases::{
    CreateUserUseCase, DeleteUserUseCase, GetUserUseCase, ListUsersUseCase, SignupUseCase,
    VerifyEmailUseCase,
};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub create_user_use_case: Arc<CreateUserUseCase>,
    pub get_user_use_case: Arc<GetUserUseCase>,
    pub delete_user_use_case: Arc<DeleteUserUseCase>,
    pub list_users_use_case: Arc<ListUsersUseCase>,
    pub signup_use_case: Arc<SignupUseCase>,
    pub verify_email_use_case: Arc<VerifyEmailUseCase>,
}

impl AppState {
    pub fn new(
        db: DatabaseConnection,
        create_user_use_case: CreateUserUseCase,
        get_user_use_case: GetUserUseCase,
        delete_user_use_case: DeleteUserUseCase,
        list_users_use_case: ListUsersUseCase,
        signup_use_case: SignupUseCase,
        verify_email_use_case: VerifyEmailUseCase,
    ) -> Self {
        Self {
            db,
            create_user_use_case: Arc::new(create_user_use_case),
            get_user_use_case: Arc::new(get_user_use_case),
            delete_user_use_case: Arc::new(delete_user_use_case),
            list_users_use_case: Arc::new(list_users_use_case),
            signup_use_case: Arc::new(signup_use_case),
            verify_email_use_case: Arc::new(verify_email_use_case),
        }
    }
}
