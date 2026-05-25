use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub rpc: RpcSettings,
}

impl Settings {
    /// Loads the application configuration settings.
    ///
    /// This method builds the configuration by layering multiple sources:
    /// 1. A `.env` file (if present, using `dotenvy`).
    /// 2. The default configuration file `config/default`.
    /// 3. An optional local override file `config/local.toml`.
    /// 4. Environment variables prefixed with `APP__`.
    ///
    /// # Errors
    ///
    /// Returns a [`ConfigError`] if the configuration files cannot be found or parsed,
    /// or if the resulting configuration cannot be successfully deserialized into [`Settings`].
    pub fn load() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();

        let config = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name("config/local.toml").required(false))
            .add_source(
                Environment::with_prefix("APP")
                    .prefix_separator("__")
                    .separator("__")
                    .try_parsing(true)
                    .list_separator(",")
                    .with_list_parse_key("server.cors_allowed_origins"),
            )
            .build()?;

        let settings: Self = config.try_deserialize()?;

        Ok(settings)
    }

    #[must_use]
    pub fn rpc_endpoint(&self) -> String {
        format!("{}{}", self.rpc.url, self.rpc.api_key)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct RpcSettings {
    pub api_key: String,
    pub url: String,
    pub rps: u32,
    pub max_concurrent: usize,
    pub max_rate_limit_retries: usize,
}
