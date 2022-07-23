use config::{Config, ConfigError, File};
use serde_aux::field_attributes::deserialize_number_from_string;
use std::convert::{TryFrom, TryInto};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
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
        .build()?;

    settings.try_deserialize()
}
