use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
pub struct Config {
    log_level: String,
    logs_json_file_path: String,
}

impl Config {
    /*  forgive the "RUST_LOG" hack; i really wanted to use the envar project-prefix to set the log-level,
    ...and couldn't figure out how to specify an alternative prefix for env_logger::init() */
    pub fn new() -> Config {
        match envy::prefixed("LOG_ERRORCHECKER__").from_env::<Config>() {
            Ok(config) => {
                env::set_var("RUST_LOG", &config.log_level);
                let log_level = config.log_level; // not yet used, but still useful to set, for panic-message if it's missing
                let logs_json_file_path = config.logs_json_file_path;
                Config {
                    log_level,
                    logs_json_file_path,
                }
            }
            Err(error) => panic!("{:#?}", error), // this shows the missing envar
        }
    }
}
