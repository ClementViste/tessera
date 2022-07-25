use config::{Config, ConfigError, Environment, File};
use secrecy::{ExposeSecret, Secret};
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    ConnectOptions, PgPool,
};
use std::convert::{TryFrom, TryInto};

#[derive(Clone, serde::Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}

#[derive(Clone, serde::Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

#[derive(Clone, serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub database_name: String,
    pub require_ssl: bool,
}

impl DatabaseSettings {
    /// Return the connection string for a Postgres instance.
    pub fn without_db(&self) -> PgConnectOptions {
        // If SSL is required, try an encrypted connection,
        // fall back to unencrypted if it fails.
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .username(&self.username)
            .password(self.password.expose_secret())
            .host(&self.host)
            .port(self.port)
            .ssl_mode(ssl_mode)
    }

    /// Return the connection string for a Postgres database.
    pub fn with_db(&self) -> PgConnectOptions {
        // Get the connection string for a Postgres database.
        let mut options = self.without_db().database(&self.database_name);

        // Remove noise.
        options.log_statements(tracing::log::LevelFilter::Trace);

        options
    }

    /// Return the connection pool for a Postgres database.
    pub fn get_connection_pool(&self) -> PgPool {
        PgPoolOptions::new()
            // Timeout after 2 seconds.
            .acquire_timeout(std::time::Duration::from_secs(2))
            // Establish a connection when the pool is used for the first time.
            .connect_lazy_with(self.with_db())
    }
}

pub enum RuntimeEnvironment {
    Local,
    Production,
}

impl RuntimeEnvironment {
    /// Extract a string slice containing the entire `String`.
    pub fn as_str(&self) -> &'static str {
        match self {
            RuntimeEnvironment::Local => "local",
            RuntimeEnvironment::Production => "production",
        }
    }
}

impl TryFrom<String> for RuntimeEnvironment {
    type Error = String;

    /// Perform the conversion.
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "`{}` is not a supported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}

/// Return the configuration values.
pub fn get_configuration() -> Result<Settings, ConfigError> {
    // Determine the location of the configuration folder.
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    // Get the runtime environment.
    //
    // Default to `local`.
    let environment: RuntimeEnvironment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse `APP_ENVIRONMENT` environment variable");

    let settings = Config::builder()
        // Read the default configuration settings.
        .add_source(File::from(configuration_directory.join("base")).required(true))
        // Layer on the environment-specific values.
        .add_source(File::from(configuration_directory.join(environment.as_str())).required(true))
        // Layer on DigitalOcean's specific values.
        .add_source(
            Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings.try_deserialize()
}
