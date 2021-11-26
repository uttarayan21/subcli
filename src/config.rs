use crate::error::Result;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub username: String,
    pub password: String,
    pub server: String,
    #[serde(default = "default_bitrate")]
    pub bitrate: usize,
}

fn default_bitrate() -> usize {
    192_usize
}

impl Config {
    pub fn try_load() -> Result<Self> {
        let name = env!("CARGO_PKG_NAME");
        let mut path = dirs::config_dir().unwrap();
        path.push(name);
        path.push("config.toml");

        // let config_file = std::fs::File::open(path)?;
        let config_file_buffer = std::fs::read(path)?;
        Ok(toml::from_slice(&config_file_buffer)?)
    }
}
