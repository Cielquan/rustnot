use crate::settings;

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use thiserror::Error;

const SETTINGS_FILE_NAME: &str = "rustnot_config.toml";

#[derive(Error, Debug, Clone)]
pub enum SettingsFileError {
    #[error("Failed to read the settings file.")]
    Read,
    #[error("Failed to write the settings file.")]
    Write,
    #[error("File does not exist.")]
    MissingFile,
    #[error("Failed to parse the settings file.")]
    ParseFile,
    #[error("Failed to parse the settings state.")]
    ParseState,
}

impl settings::Settings {
    pub fn load_from_file() -> Result<Self, SettingsFileError> {
        if !Path::new(SETTINGS_FILE_NAME).exists() {
            return Err(SettingsFileError::MissingFile);
        }

        let setttings_string = match fs::read_to_string(SETTINGS_FILE_NAME) {
            Err(_) => return Err(SettingsFileError::Read),
            Ok(s) => s,
        };

        match toml::from_str(&setttings_string) {
            Err(_) => Err(SettingsFileError::ParseFile),
            Ok(s) => Ok(s),
        }
    }

    pub fn save_to_file(&self) -> Result<(), SettingsFileError> {
        let settings_string = match toml::to_string_pretty(&self) {
            Err(_) => return Err(SettingsFileError::ParseState),
            Ok(s) => s,
        };

        let mut file = match File::create(SETTINGS_FILE_NAME) {
            Err(_) => return Err(SettingsFileError::Write),
            Ok(f) => f,
        };

        match write!(file, "{}", settings_string) {
            Err(_) => Err(SettingsFileError::Write),
            Ok(_) => Ok(()),
        }
    }
}
