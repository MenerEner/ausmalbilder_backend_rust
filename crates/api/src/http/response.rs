use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ApiResponse<T> {
    #[schema(example = true, default = true)]
    pub success: bool,
    pub data: T,
}

#[derive(Serialize, ToSchema)]
pub struct PaginatedResponse<T> {
    pub success: bool,
    pub data: Vec<T>,
    pub pagination: PaginationInfo,
}

#[derive(Serialize, ToSchema)]
pub struct PaginationInfo {
    pub page: u64,
    pub page_size: u64,
    pub total_items: u64,
    pub total_pages: u64,
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

impl<T> PaginatedResponse<T> {
    pub fn success(data: Vec<T>, page: u64, page_size: u64, total_items: u64) -> Self {
        let total_pages = (total_items as f64 / page_size as f64).ceil() as u64;
        Self {
            success: true,
            data,
            pagination: PaginationInfo {
                page,
                page_size,
                total_items,
                total_pages,
            },
        }
    }
}
