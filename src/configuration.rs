pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialise our configuration reader
    let mut settings = config::Config::default();
    // Add configuration values from a file named `configuration`. // It will look for any top-level file with an extension
    // that `config` knows how to parse: yaml, json, etc.
    settings.merge(config::File::with_name("resources/configuration"))?;
    settings.try_into()
}

#[derive(serde::Deserialize)]
pub struct Settings {
    pub host_name: String
}