use dotenv;
use std::path::Path;

pub struct ConnectionString(String);

impl ConnectionString {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub struct LocalizationRoot(String);
impl LocalizationRoot {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub struct AppConfig {
    pub connection_string: ConnectionString,
    pub localization_root: LocalizationRoot,
}

pub fn load_config(file: Option<&Path>) -> AppConfig {
    match file {
        None => {
            dotenv::dotenv().ok();
        }
        Some(path) => {
            dotenv::from_path(path).expect("TODO: panic message");
        }
    }

    AppConfig {
        connection_string: ConnectionString(
            std::env::var("DATABASE_URL").expect("there is no DATABASE_URL in env vars"),
        ),
        localization_root: LocalizationRoot(
            std::env::var("LOCALIZATION_ROOT").expect("there is no LOCALIZATION_ROOT in env vars"),
        ),
    }
}
