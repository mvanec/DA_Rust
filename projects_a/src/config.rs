use config::{ConfigError, File};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    database_url: String,
    projects_csv: String,
    tasks_csv: String,
    timings_csv: String,
}

impl Config {
    pub fn new() -> Self {
        let mut config = config::Config::default();
        config
            .merge(File::with_name("config"))
            .expect("Failed to read configuration file");
        config.try_into().expect("Failed to parse configuration file")
    }

    pub fn database_url(&self) -> &str {
        &self.database_url
    }

    pub fn projects_csv(&self) -> &str {
        &self.projects_csv
    }

    pub fn tasks_csv(&self) -> &str {
        &self.tasks_csv
    }

    pub fn timings_csv(&self) -> &str {
        &self.timings_csv
    }
}
