pub use configure::ConfigureError;
use serde::{Deserialize, Serialize};
pub use status::Status;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum SocketRequest {
    Configure { setting: String, value: String },
    Status,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum SocketResponse {
    Configure(Result<(), ConfigureError>),
    Status(Status),
}

mod configure {
    use core::fmt::{self, Formatter};
    use serde::{Deserialize, Serialize};
    use std::error::Error;

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub enum ConfigureError {
        UnknownSetting,
    }

    impl fmt::Display for ConfigureError {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                Self::UnknownSetting => write!(f, "unknown setting"),
            }
        }
    }

    impl Error for ConfigureError {}
}

mod status {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status {
        pub dashboard_hostname: String,
        pub registry_hostname: String,
    }
}
