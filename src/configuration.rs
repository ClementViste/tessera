use config::{Config, ConfigError, File};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

/// Representation of the settings.
#[derive(Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
}

/// Representation of the application's settings.
#[derive(Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
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
