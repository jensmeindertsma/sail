use core::fmt::{self, Formatter};

pub enum Command {
    Create { name: String },
    Help,
    List,
    SetDashboardHost(String),
    SetRegistryHost(String),
}

impl Command {
    pub fn try_from_arguments(
        arguments: impl IntoIterator<Item = String>,
    ) -> Result<Self, CommandParseError> {
        let mut iterator = arguments.into_iter();

        match iterator
            .next()
            .ok_or(CommandParseError::NoArguments)?
            .as_str()
        {
            "help" => Ok(Self::Help),
            "list" => Ok(Self::List),
            "create" => {
                let name = iterator
                    .next()
                    .ok_or(CommandParseError::MissingCreateName)?;

                Ok(Self::Create { name })
            }
            "set-dashboard-host" => {
                let host = iterator.next().ok_or(CommandParseError::NoArguments)?;

                Ok(Self::SetDashboardHost(host))
            }
            "set-registry-host" => {
                let host = iterator.next().ok_or(CommandParseError::NoArguments)?;

                Ok(Self::SetRegistryHost(host))
            }
            other => Err(CommandParseError::UnknownCommand(other.to_owned())),
        }
    }
}

pub enum CommandParseError {
    MissingCreateName,
    NoArguments,
    UnknownCommand(String),
}

impl fmt::Display for CommandParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingCreateName => write!(f, "missing required argument <app_name>"),
            Self::NoArguments => write!(f, "no arguments were provided"),
            Self::UnknownCommand(cmd) => write!(f, "unknown command `{cmd}`"),
        }
    }
}
