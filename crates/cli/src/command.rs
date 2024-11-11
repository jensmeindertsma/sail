pub mod update;

use core::fmt::{self, Formatter};
use std::error::Error;

pub enum Command {
    Configure { setting: String, value: String },
    Update,
    Status,
}

impl Command {
    pub fn parse(mut arguments: impl Iterator<Item = String>) -> Result<Self, ParseError> {
        let command_argument = arguments.next().ok_or(ParseError::MissingCommand)?;

        match command_argument.as_str() {
            "configure" => {
                let setting = arguments
                    .next()
                    .ok_or(ParseError::MissingArgument { name: "setting" })?;

                let value = arguments
                    .next()
                    .ok_or(ParseError::MissingArgument { name: "value" })?;

                Ok(Self::Configure { setting, value })
            }
            "status" => Ok(Self::Status),
            "update" => Ok(Self::Update),
            _ => Err(ParseError::UnknownCommand(command_argument)),
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    MissingArgument { name: &'static str },
    MissingCommand,
    UnknownCommand(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingArgument { name } => write!(f, "missing argument <{name}>"),
            Self::MissingCommand => write!(f, "missing required command argument"),
            Self::UnknownCommand(command) => write!(f, "unknown command `{command}`"),
        }
    }
}

impl Error for ParseError {}
