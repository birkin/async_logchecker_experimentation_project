use chrono::{DateTime, Local};
use serde::Deserialize;
use tokio::time::{Duration, Instant};
use std::env;


#[derive(Deserialize, Debug)]
struct Config {
    log_level: String,
    logger_json_file_path: String,
    max_entries: i8  // this could be added to the json-file instead
}

impl Config {
    /*  forgive the "RUST_LOG" hack; i really wanted to use the envar project-prefix to set the log-level,
        ...and couldn't figure out how to specify an alternative prefix for env_logger::init() */
    fn new() -> Config {
        match envy::prefixed("LOG_ERRORCHECKER__").from_env::<Config>() {  // https://github.com/softprops/envy
            Ok(config) => {
                env::set_var( "RUST_LOG", &config.log_level);
                let log_level = config.log_level;  // not used, but still useful to set, for panic-message if it's missing
                let logger_json_file_path = config.logger_json_file_path;
                let max_entries = config.max_entries;
                Config { log_level, logger_json_file_path, max_entries }
            },
            Err(error) => panic!("{:#?}", error) // this shows the missing envar
        }
    }
}

#[tokio::main]
async fn main() {

    /* start */
    let start_time = Instant::now();
    println!( "start_time, ``{:?}``", start_time );
    // let local_time: DateTime<Local> = Local::now().await?;  // used for logging
    let local_time = Local::now();  // used for logging
    println!( "local_time, ``{:?}``", local_time );
    println!( "Hello, world!" );
}

// fn main() {
//     println!("Hello, world!");
// }
