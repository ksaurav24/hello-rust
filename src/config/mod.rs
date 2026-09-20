use serde::Deserialize;
use std::{env, u16};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
// add display trait

pub enum Environment {
    Development,
    Production,
}

#[derive(Clone, Deserialize)]
pub struct Config {
    pub port: u16,
    pub environment: Environment,
}

pub trait ConfigLoader {
    fn load_env() -> Config;
}

impl ConfigLoader for Config {
    fn load_env() -> Self {
        dotenvy::dotenv().ok();

        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_owned())
            .parse::<u16>()
            .expect("PORT must be a valid number");

        let environment = match env::var("ENVIRONMENT")
            .unwrap_or_else(|_| "development".to_string())
            .as_str()
        {
            "development" => Environment::Development,
            "production" => Environment::Production,
            _ => panic!("Invalid ENVIRONMENT"),
        };

        Self { port, environment }
    }
}
