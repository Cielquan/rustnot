use std::fmt::Display;
use std::fs::{self, File};
use std::io::Write;

use thiserror::Error;

/// The configuration object.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Config {
    pub sit_time: u32,
    pub stand_time: u32,
    pub start_stance: Stance,
    pub toast_duration: u32,
}

/// The application spec
impl Default for Config {
    fn default() -> Self {
        toml::from_str(DEFAULT_CONFIG_TOML_STR).unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Stance {
    Standing,
    Sitting,
}

impl Default for Stance {
    fn default() -> Self {
        Self::Sitting
    }
}

impl Display for Stance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stance::Sitting => write!(f, "{}", "Sitting"),
            Stance::Standing => write!(f, "{}", "Standing"),
        }
    }
}

#[derive(Error, Debug, Clone)]
pub enum ConfigFileError {
    #[error("Failed to read the config file.")]
    Read,
    #[error("Failed to write the config file.")]
    Write,
    #[error("Failed to parse the config file.")]
    ParseFile,
    #[error("Failed to parse the config state.")]
    ParseState,
}

impl Config {
    pub async fn load_from_file() -> Result<Config, ConfigFileError> {
        let conf_string = match fs::read_to_string(CONFIG_FILE_PATH) {
            Err(_) => {
                return Err(ConfigFileError::Read);
            }
            Ok(c) => c,
        };
        match toml::from_str::<Config>(conf_string.as_str()) {
            Err(_) => Err(ConfigFileError::ParseFile),
            Ok(c) => Ok(c),
        }
    }

    pub async fn save_to_file(conf: Config) -> Result<(), ConfigFileError> {
        let conf_string = match toml::to_string(&conf) {
            Err(_) => {
                return Err(ConfigFileError::ParseState);
            }
            Ok(c) => c,
        };
        let mut file = match File::create(CONFIG_FILE_PATH) {
            Err(_) => {
                return Err(ConfigFileError::Write);
            }
            Ok(f) => f,
        };
        match write!(file, "{}", conf_string.as_str()) {
            Err(_) => {
                return Err(ConfigFileError::Write);
            }
            Ok(_) => Ok(()),
        }
    }
}

/// Default configuration of rustnot as TOML string.
const DEFAULT_CONFIG_TOML_STR: &str = r#"
sit_time = 45
stand_time = 15
start_stance = "Sitting"
toast_duration = 7
"#;

const CONFIG_FILE_PATH: &str = "rustnot_config.toml";
