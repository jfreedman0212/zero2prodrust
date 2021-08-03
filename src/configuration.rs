use config::ConfigError;

/// Represents the total configuration for the application
#[derive(serde::Deserialize)]
pub struct Settings {
    /// Contains database settings (see type for more details)
    pub database: DatabaseSettings,
    /// Port that the application will listen on
    pub application_port: u16,
}

/// Contains all settings necessary to communicate with a Database
#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    /// Database Username
    pub username: String,
    /// Database Password
    pub password: String,
    /// Port the database is listening on
    pub port: u16,
    /// Hostname that the database is hosted on
    pub host: String,
    /// Name of the database to connect to
    pub database_name: String,
}

impl DatabaseSettings {
    /// Constructs a full Postgres connection string
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    /// Constructs a Postgres connection string without the DB name
    /// To use this, you'll need to provide a name to use
    pub fn connection_string_without_db_name(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

/// Reads from a "configuration.yml" file and returns a `Result` containing either
/// the populated `Settings` or an error reading from the config file.
pub fn get_configuration() -> Result<Settings, ConfigError> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("configuration"))?;
    settings.try_into()
}
