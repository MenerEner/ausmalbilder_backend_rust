use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ApiResponse<T> {
    #[schema(example = true, default = true)]
    pub success: bool,
    pub data: T,
}

#[derive(Serialize, ToSchema)]
pub struct ApiResponseUser {
    #[schema(example = true, default = true)]
    pub success: bool,
    pub data: crate::http::users::dtos::UserResponse,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data,
        }
    }
}
