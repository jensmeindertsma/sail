use std::{env, os::fd::FromRawFd};
use tokio::net::UnixListener;

pub fn create_socket() -> UnixListener {
    let listen_fds = env::var("LISTEN_FDS")
        .ok()
        .and_then(|v| v.parse::<i32>().ok())
        .unwrap_or(0);

    if listen_fds == 0 {
        tracing::error!("no socket passed from systemd");
        panic!("No socket passed from systemd");
    }

    tracing::info!("received socket file descriptor from systemd");

    // SAFETY: systemd guarantees the FD is valid
    let std_listener = unsafe { std::os::unix::net::UnixListener::from_raw_fd(3) };
    std_listener.set_nonblocking(true).unwrap();

    UnixListener::from_std(std_listener).unwrap()
}
