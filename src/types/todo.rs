use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateListRequest {
    pub name: String,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateListResponse {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetListRequest {
    pub name: String,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetListResponse {
    pub name: String,
}
