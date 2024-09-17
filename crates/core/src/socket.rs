use crate::configuration::Application;
use serde::{Deserialize, Serialize};
use std::net::SocketAddrV4;

#[derive(Deserialize, Serialize)]
pub struct SocketMessage {
    pub id: u8,
    pub request: SocketRequest,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SocketRequest {
    CreateApplication {
        name: String,
        hostname: String,
        address: SocketAddrV4,
    },
    EditApplication {
        name: String,
        new: Application,
    },
    GetApplication {
        name: String,
    },
    DeleteApplication {
        name: String,
    },
    ListApplications,
}

#[derive(Deserialize, Serialize)]
pub struct SocketReply {
    pub regarding: u8,
    pub response: SocketResponse,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SocketResponse {
    Success(SuccessResponse),
    Failure(FailureReason),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SuccessResponse {
    ListApplications(Vec<Application>),
    CreatedApplication { name: String },
    GotApplication(Application),
    EditedApplication { name: String },
    DeletedApplication { name: String },
}

#[derive(Debug, Deserialize, Serialize)]
pub enum FailureReason {
    ApplicationNotFound,
    HostnameInUse,
    NameInUse,
    Todo,
}
