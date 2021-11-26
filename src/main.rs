#[macro_use]
extern crate log;

use chrono::Local;
// use chrono::{DateTime, Local};
use tokio::time::Instant;
// use tokio::time::{Duration, Instant};

use env_logger::{Builder, Target};

use logchecker_project::Config;

#[tokio::main]
async fn main() {
    /* start */
    let start_time = Instant::now();
    println!("start_time, ``{:?}``", start_time);
    let local_time = Local::now(); // used for logging
                                   // println!("local_time, ``{:?}``", local_time);
                                   // println!("Hello, world!");

    /* load settings */
    let config = Config::new();
    println!("config, ``{:?}``", config);

    /* setup logging */
    let mut log_builder = Builder::from_default_env();
    log_builder.target(Target::Stdout);
    log_builder.init();
    let msg: String = format!(
        "\n\n-------\n`starting logchecker_project code at, ``{:?}``",
        local_time.to_rfc3339()
    );
    info!("{}", msg)

    /* load log_paths.json file (sync) */

    /* get list of candidate files (async) */

    /* process each candidate file, saving output (async) */

    /* massage output and email (sync) */
}
