use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use parking_lot::Mutex;

/// The configuration object.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Config {
    pub settings: Settings,
}

/// The application specific part of the confiuration.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Settings {
    pub sit_time: u64,
    pub stand_time: u64,
}

/// Default configuration of rustnot as TOML string.
const DEFAULT_CONFIG_TOML_STR: &str = r#"
settings.sit_time = 45
settings.stand_time = 15
"#;

const CONFIG_FILE_PATH: &str = "rustnot_config.toml";

// NOTE: `unwrap` shall never trigger as the string above is always the same
lazy_static! {
    /// The current config used by rustnot.
    #[derive(Debug)]
    pub static ref CONFIG: Mutex<Config> = Mutex::new(toml::from_str(DEFAULT_CONFIG_TOML_STR).unwrap());
}

pub fn load_config() -> anyhow::Result<()> {
    if Path::new(CONFIG_FILE_PATH).is_file() {
        let conf_string = fs::read_to_string(CONFIG_FILE_PATH)?;
        let conf: Config = toml::from_str(conf_string.as_str())?;
        *CONFIG.lock() = conf;
    }
    Ok(())
}

pub fn save_config() -> anyhow::Result<()> {
    let conf_string = toml::to_string(&*CONFIG.lock())?;
    let mut file = File::create(CONFIG_FILE_PATH)?;
    write!(file, "{}", conf_string.as_str())?;
    Ok(())
}
