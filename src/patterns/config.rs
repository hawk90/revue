//! Configuration loading utilities
//!
//! Provides standard patterns for loading TOML configuration files
//! from XDG-compliant locations (~/.config/app-name/config.toml).
//!
//! # Example
//!
//! ```ignore
//! use revue::patterns::{AppConfig, ConfigError};
//! use serde::Deserialize;
//!
//! #[derive(Deserialize)]
//! struct MyAppConfig {
//!     server_url: String,
//!     api_token: String,
//!     #[serde(default)]
//!     timeout: u64,
//! }
//!
//! impl AppConfig for MyAppConfig {
//!     fn config_dir() -> &'static str {
//!         "myapp"
//!     }
//!
//!     fn config_file() -> &'static str {
//!         "config.toml"
//!     }
//! }
//!
//! fn main() {
//!     let config = MyAppConfig::load().unwrap_or_else(|e| {
//!         eprintln!("Config error: {}", e);
//!         std::process::exit(1);
//!     });
//! }
//! ```

use serde::de::DeserializeOwned;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

/// Configuration loading errors
#[derive(Debug, Clone)]
pub enum ConfigError {
    /// Config file not found
    NotFound(PathBuf),
    /// Home directory not found
    NoHome,
    /// Failed to read file
    ReadError(PathBuf, String),
    /// Failed to parse TOML
    ParseError(String),
    /// Invalid configuration value
    ValidationError(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::NotFound(path) => {
                write!(f, "Config file not found: {}", path.display())
            }
            ConfigError::NoHome => {
                write!(f, "Could not determine home directory")
            }
            ConfigError::ReadError(path, err) => {
                write!(f, "Failed to read {}: {}", path.display(), err)
            }
            ConfigError::ParseError(err) => {
                write!(f, "Failed to parse config: {}", err)
            }
            ConfigError::ValidationError(err) => {
                write!(f, "Invalid config: {}", err)
            }
        }
    }
}

impl std::error::Error for ConfigError {}

/// Configuration loading trait
///
/// Implement this for your app's config struct to get standardized loading.
pub trait AppConfig: Sized + DeserializeOwned {
    /// Config directory name under ~/.config/
    ///
    /// # Example
    ///
    /// ```ignore
    /// fn config_dir() -> &'static str {
    ///     "myapp"  // -> ~/.config/myapp/
    /// }
    /// ```
    fn config_dir() -> &'static str;

    /// Config file name
    ///
    /// # Example
    ///
    /// ```ignore
    /// fn config_file() -> &'static str {
    ///     "config.toml"
    /// }
    /// ```
    fn config_file() -> &'static str {
        "config.toml"
    }

    /// Get full config file path
    ///
    /// Returns `~/.config/{config_dir}/{config_file}`
    fn config_path() -> Result<PathBuf, ConfigError> {
        let home = dirs::home_dir().ok_or(ConfigError::NoHome)?;
        Ok(home
            .join(".config")
            .join(Self::config_dir())
            .join(Self::config_file()))
    }

    /// Load config from default location
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Home directory not found
    /// - Config file doesn't exist
    /// - File can't be read
    /// - TOML parsing fails
    fn load() -> Result<Self, ConfigError> {
        let path = Self::config_path()?;
        Self::load_from(&path)
    }

    /// Load config from specific path
    fn load_from(path: &Path) -> Result<Self, ConfigError> {
        if !path.exists() {
            return Err(ConfigError::NotFound(path.to_path_buf()));
        }

        let contents = fs::read_to_string(path)
            .map_err(|e| ConfigError::ReadError(path.to_path_buf(), e.to_string()))?;

        toml::from_str(&contents).map_err(|e| ConfigError::ParseError(e.to_string()))
    }

    /// Load config or use default
    ///
    /// # Example
    ///
    /// ```ignore
    /// let config = MyAppConfig::load_or_default();
    /// ```
    fn load_or_default() -> Self
    where
        Self: Default,
    {
        Self::load().unwrap_or_default()
    }

    /// Load config or exit with error message
    ///
    /// Convenience method for simple CLIs.
    ///
    /// # Example
    ///
    /// ```ignore
    /// fn main() {
    ///     let config = MyAppConfig::load_or_exit();
    ///     // ...
    /// }
    /// ```
    fn load_or_exit() -> Self {
        Self::load().unwrap_or_else(|e| {
            eprintln!("Configuration error: {}", e);
            if let ConfigError::NotFound(path) = e {
                eprintln!("\nCreate config file at: {}", path.display());
                eprintln!("\nExample config:");
                eprintln!("{}", Self::example_config());
            }
            std::process::exit(1);
        })
    }

    /// Validate configuration
    ///
    /// Override to add custom validation logic.
    ///
    /// # Example
    ///
    /// ```ignore
    /// fn validate(&self) -> Result<(), ConfigError> {
    ///     if self.api_token.is_empty() {
    ///         return Err(ConfigError::ValidationError(
    ///             "api_token cannot be empty".to_string()
    ///         ));
    ///     }
    ///     Ok(())
    /// }
    /// ```
    fn validate(&self) -> Result<(), ConfigError> {
        Ok(())
    }

    /// Example config string
    ///
    /// Override to provide example config for error messages.
    fn example_config() -> &'static str {
        "# Add example config here"
    }
}

/// Helper to create config directory if it doesn't exist
///
/// # Example
///
/// ```ignore
/// ensure_config_dir("myapp")?;
/// ```
pub fn ensure_config_dir(app_name: &str) -> Result<PathBuf, ConfigError> {
    let home = dirs::home_dir().ok_or(ConfigError::NoHome)?;
    let config_dir = home.join(".config").join(app_name);

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)
            .map_err(|e| ConfigError::ReadError(config_dir.clone(), e.to_string()))?;
    }

    Ok(config_dir)
}

/// Helper to write default config file
///
/// # Example
///
/// ```ignore
/// write_default_config("myapp", "config.toml", DEFAULT_CONFIG_TOML)?;
/// ```
pub fn write_default_config(
    app_name: &str,
    filename: &str,
    contents: &str,
) -> Result<PathBuf, ConfigError> {
    let config_dir = ensure_config_dir(app_name)?;
    let config_path = config_dir.join(filename);

    if config_path.exists() {
        return Err(ConfigError::ValidationError(format!(
            "Config file already exists: {}",
            config_path.display()
        )));
    }

    fs::write(&config_path, contents)
        .map_err(|e| ConfigError::ReadError(config_path.clone(), e.to_string()))?;

    Ok(config_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize, Default)]
    struct TestConfig {
        name: String,
        value: i32,
    }

    impl AppConfig for TestConfig {
        fn config_dir() -> &'static str {
            "revue-test"
        }

        fn example_config() -> &'static str {
            "name = \"test\"\nvalue = 42"
        }
    }

    #[test]
    fn test_config_path() {
        let path = TestConfig::config_path().unwrap();
        assert!(path
            .to_string_lossy()
            .contains(".config/revue-test/config.toml"));
    }

    #[test]
    fn test_load_from_string() {
        let toml = "name = \"test\"\nvalue = 42";
        let config: TestConfig = toml::from_str(toml).unwrap();
        assert_eq!(config.name, "test");
        assert_eq!(config.value, 42);
    }

    #[test]
    fn test_config_error_display() {
        let err = ConfigError::NotFound(PathBuf::from("/test/config.toml"));
        assert!(err.to_string().contains("/test/config.toml"));

        let err = ConfigError::NoHome;
        assert!(err.to_string().contains("home directory"));

        let err = ConfigError::ParseError("invalid TOML".to_string());
        assert!(err.to_string().contains("invalid TOML"));
    }
}
