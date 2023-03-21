use config::{Config, ConfigError, File};
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    ConnectOptions, PgPool,
};

/// Representation of the settings.
#[derive(Clone, Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}

/// Representation of the application's settings.
#[derive(Clone, Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

/// Representation of the database's settings.
#[derive(Clone, Deserialize)]
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
    /// Returns the connection string for a Postgres instance.
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

    /// Returns the connection string for a Postgres database.
    pub fn with_db(&self) -> PgConnectOptions {
        let mut options = self.without_db().database(&self.database_name);

        // Remove noise.
        options.log_statements(tracing::log::LevelFilter::Trace);

        options
    }

    /// Returns the connection pool for a Postgres database.
    pub fn get_connection_pool(&self) -> PgPool {
        PgPoolOptions::new()
            // Timeout after 2 seconds.
            .acquire_timeout(std::time::Duration::from_secs(2))
            // Establish a connection when the pool is used for the first time.
            .connect_lazy_with(self.with_db())
    }
}

/// Representation of the runtime environments.
pub enum RuntimeEnvironment {
    Development,
    Production,
}

impl RuntimeEnvironment {
    /// Extracts a string slice containing the entire `String`.
    fn as_str(&self) -> &'static str {
        match self {
            RuntimeEnvironment::Development => "development",
            RuntimeEnvironment::Production => "production",
        }
    }
}

impl TryFrom<String> for RuntimeEnvironment {
    type Error = String;

    /// Performs the conversion.
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "development" => Ok(Self::Development),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "`{other}` is not a supported environment. Use either `development` or `production`."
            )),
        }
    }
}

/// Returns the configuration values.
pub fn get_configuration() -> Result<Settings, ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    // Get the runtime environment.
    //
    // Default to `development`.
    let environment: RuntimeEnvironment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "development".into())
        .try_into()
        .expect("Failed to parse `APP_ENVIRONMENT` environment variable");

    let settings = Config::builder()
        // Read the default configuration settings.
        .add_source(File::from(configuration_directory.join("base")).required(true))
        // Layer on the environment-specific values.
        .add_source(File::from(configuration_directory.join(environment.as_str())).required(true))
        .build()?;

    settings.try_deserialize()
}
