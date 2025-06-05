use std::{fs, path::Path};
use tokio::net::UnixListener;

pub fn create_socket() -> UnixListener {
    if Path::new("/run/sail.socket").exists() {
        fs::remove_file("/run/sail.socket").unwrap();
    }

    // TODO: better permissionss

    UnixListener::bind("/run/sail.socket").unwrap()
}
