pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialise our configuration reader
    let mut settings = config::Config::default();
    // Add configuration values from a file named `configuration`. // It will look for any top-level file with an extension
    // that `config` knows how to parse: yaml, json, etc.
    settings.merge(config::File::with_name("configuration"))?;
    settings.try_into()
}

#[derive(serde::Deserialize)]
pub struct Settings {
    pub protocol: String,
    pub host_name: String,
    pub mod_nm: String,
    pub filler: String,
    pub set_nm: u8,
    pub joiner: String,
    pub mod_append: String,
    pub file_suffix: String,
}

impl Settings {
    pub fn get_url_to_append_set(&self) -> String {
        format!(
            "{}://{}/{}/{}",
            self.protocol, self.host_name, self.mod_nm, self.filler
        )
    }
}
