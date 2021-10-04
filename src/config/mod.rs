use serde::{Serialize, Deserialize};
use serde_json;

/// Example: {"db_connect":"host=localhost user=test123 password=test123"}
/// You have to store test123 user and test123 database to make it work correctly.
#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub db_connect: String
}

impl Config {
    pub fn generate(self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn from_file(path: &str) -> Result<Config, std::io::Error> {
        let buf = std::fs::read_to_string(path)?;
        let s: Self = serde_json::from_str(&buf).unwrap();
        Ok(s)
    }
}
