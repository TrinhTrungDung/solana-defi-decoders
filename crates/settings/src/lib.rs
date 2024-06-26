use std::{env, path::Path};

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use tracing::level_filters::LevelFilter;

#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    pub tracing: Tracing,
    pub solana: Solana,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Tracing {
    pub debug: bool,
    pub dirname: String,
    pub prefix: String,
    pub level: String,
}

impl Tracing {
    pub fn to_level_filter(&self) -> LevelFilter {
        match self.level.as_str() {
            "debug" => LevelFilter::DEBUG,
            "error" => LevelFilter::ERROR,
            "info" => LevelFilter::INFO,
            "trace" => LevelFilter::TRACE,
            "warn" => LevelFilter::WARN,
            _ => LevelFilter::OFF,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Solana {
    pub wss: String,
    pub rpc: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap();
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            .add_source(File::with_name(
                root_dir.join("config/default").to_str().unwrap(),
            ))
            .add_source(
                File::with_name(
                    root_dir
                        .join(format!("config/{}", run_mode))
                        .to_str()
                        .unwrap(),
                )
                .required(false),
            )
            .add_source(
                File::with_name(root_dir.join("config/local").to_str().unwrap()).required(false),
            )
            .add_source(Environment::with_prefix("app"))
            .build()?;

        s.try_deserialize()
    }
}
