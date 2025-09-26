use serde::{Serialize, Deserialize};
pub mod add_key;

#[derive(Serialize, Deserialize)]
pub struct ResponseData {
    pub command: Option<String>,
    pub success: bool,
    pub fingerprint: Option<u32>,
    pub error: Option<String>,
    pub error_details: Option<ErrorDetails>,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorDetails {
    pub message: String,
}