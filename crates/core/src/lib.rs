use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SocketRequest {
    FetchGreeting,
    FetchPort,
    ModifyGreeting(String),
    ModifyPort(u16),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SocketResponse {
    FetchGreeting(SocketResult<String>),
    FetchPort(SocketResult<u16>),
    ModifyGreeting(SocketResult<()>),
    ModifyPort(SocketResult<RestartRequired>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SocketError {}

type SocketResult<T> = Result<T, SocketError>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RestartRequired;
