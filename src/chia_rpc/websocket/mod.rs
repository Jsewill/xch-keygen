use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Request<T: Serialize> {
    pub ack: bool,
    pub command: String,
    pub request_id: Option<String>,
    pub destination: String,
    pub origin: Option<String>,
    pub data: T,
}