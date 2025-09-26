use serde::{Serialize, Deserialize};
use crate::chia_rpc::daemon::ResponseData;

#[derive(Serialize, Deserialize)]
pub struct Command {
    pub kc_user: Option<String>,
    pub kc_service: Option<String>,
    pub mnemonic_or_pk: String,
    pub label: Option<String>,
    pub private: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub ack: bool,
    pub command: String,
    pub data: ResponseData,
    pub destination: String,
    pub origin: String,
    pub request_id: String,
}