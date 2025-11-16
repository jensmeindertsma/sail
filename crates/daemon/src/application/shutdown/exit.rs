use crate::application::{Failure, Task, socket::SocketError};
use tokio::task::JoinError;

pub fn handle_socket_exit(
    output: Result<Result<(), SocketError>, JoinError>,
) -> Result<(), Failure> {
    match output {
        Ok(Ok(())) => {
            tracing::info!("socket handler exited gracefully");
            Ok(())
        }
        Ok(Err(socket_error)) => {
            tracing::warn!("socket exited");
            Err(Failure::Socket(socket_error))
        }
        Err(_) => Err(Failure::Task(Task::Socket)),
    }
}
