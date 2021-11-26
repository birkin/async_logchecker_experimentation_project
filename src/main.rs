// use chrono::{DateTime, Local};
use chrono::Local;
// use tokio::time::{Duration, Instant};
use tokio::time::Instant;

use logchecker_project::Config;

#[tokio::main]
async fn main() {
    /* start */
    let start_time = Instant::now();
    println!("start_time, ``{:?}``", start_time);
    // let local_time: DateTime<Local> = Local::now().await?;  // used for logging
    let local_time = Local::now(); // used for logging
    println!("local_time, ``{:?}``", local_time);
    println!("Hello, world!");

    /* load settings */
    let config = Config::new();
    println!("config, ``{:?}``", config);
}

// fn main() {
//     println!("Hello, world!");
// }
