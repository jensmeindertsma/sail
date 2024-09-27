use crate::configuration::Application;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SocketMessage {
    pub id: u8,
    pub request: SocketRequest,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SocketRequest {
    CreateApplication {
        name: String,
        hostname: String,
    },
    GetApplication {
        name: String,
    },
    GetApplications,
    EditApplication {
        name: String,
        new_name: Option<String>,
        new_hostname: Option<String>,
    },
    DeleteApplication {
        name: String,
    },
    GetDashboardHost,
    GetRegistryHost,
    EditDashboardHost {
        hostname: String,
    },
    EditRegistryHost {
        hostname: String,
    },
}

#[derive(Deserialize, Serialize)]
pub struct SocketReply {
    pub regarding: u8,
    pub response: SocketResponse,
}

pub type SocketResponse = Result<Success, Failure>;

#[derive(Debug, Deserialize, Serialize)]
pub enum Success {
    CreatedApplication,
    DeletedApplication,
    EditedApplication,
    EditedDashboardHost,
    EditedRegistryHost,
    GetApplication(Application),
    GetApplications(Vec<Application>),
    GetDashboardHost { hostname: String },
    GetRegistryHost { hostname: String },
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Failure {
    ApplicationNotFound,
    ConnectionClosed,
    HostnameInUse,
    NameInUse,
    Todo,
}
