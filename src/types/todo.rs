use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateListRequest {
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateListResponse {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetListResponse {
    pub name: String,
}
