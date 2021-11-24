use chrono::{DateTime, Local};
use tokio::time::{Duration, Instant};


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
