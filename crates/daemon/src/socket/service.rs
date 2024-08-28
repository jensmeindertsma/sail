use super::SocketConnection;
use sail_core::socket::{SocketReply, SocketRequest, SocketResponse};
use std::{convert::Infallible, future::Future};
use tracing::{error, info};
