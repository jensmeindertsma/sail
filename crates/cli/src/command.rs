mod configure;
mod uninstall;
mod update;

pub enum Command {
    Configure { setting: String, value: String },
    Uninstall,
    Update,
}

impl Command {
    pub fn try_from_arguments(arguments: impl Iterator<Item = String>) -> Result<Self, ParseError> {
        let command_argument = arguments.next().ok_or(ParseError::MissingCommandArgument)?;

        match command_argument.as_str() {
            "configure" => Ok(Self::Configure {
                setting: arguments
                    .next()
                    .ok_or(ConfigureError::MissingSettingArgument)?,
                value: arguments
                    .next()
                    .ok_or(ConfigureError::MissingValueArgument)?,
            }),
            "uninstall" => Ok(Self::Uninstall),
            "update" => Ok(Self::Update),
            _other => Err(ParseError::UnknownCommand(command_argument)),
        }
    }
}

pub enum ParseError {
    MissingCommandArgument,
    UnknownCommand(String),
}
