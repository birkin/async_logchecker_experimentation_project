use chrono::Local;
// use chrono::{DateTime, Local};
use tokio::time::Instant;
// use tokio::time::{Duration, Instant};

use logchecker_project::Config;

#[tokio::main]
async fn main() {
    /* start */
    let start_time = Instant::now();
    println!("start_time, ``{:?}``", start_time);
    let local_time = Local::now(); // used for logging
    println!("local_time, ``{:?}``", local_time);
    println!("Hello, world!");

    /* load settings */
    let config = Config::new();
    println!("config, ``{:?}``", config);
}
