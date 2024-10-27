pub fn configure(mut arguments: impl Iterator<Item = String>) -> Result<(), ConfigureError> {
    let setting = arguments
        .next()
        .ok_or(ConfigureError::MissingSettingArgument)?;

    Ok(())
}

pub enum ConfigureError {
    MissingSettingArgument,
}
