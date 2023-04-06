use serde::Deserialize;

#[derive(Deserialize)]
pub struct Setting {
    pub database: DatabaseSettings,
    pub applicaton_port: u16,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/",
            self.username, self.password, self.host, self.port
        )
    }
}

pub fn get_configuration() -> Result<Setting, config::ConfigError> {
    // Initiaise our configuration reader
    let mut settings = config::Config::default();

    // Add configuration values from a file named `configuration`.
    // It will look for any top-level file with an extension
    // that `config` keows how to parse: yaml, json, etc.
    settings.merge(config::File::with_name("configuration"))?;

    // Try to convert the configuration vlaues it read into
    // our Settings type
    settings.try_into()
}
