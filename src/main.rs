#[macro_use]
extern crate log;

use chrono::Local;
use env_logger::{Builder, Target};
use logchecker_project::evaluate_dirs;
use logchecker_project::Config;
use logchecker_project::PathsSource;
use tokio::time::{Duration, Instant};

#[tokio::main]
async fn main() {
    /* start --------------------------------- */
    let start_time = Instant::now();
    println!("start_time, ``{:?}``", start_time);
    let local_time = Local::now(); // used for logging

    /* load settings ------------------------- */
    let config: Config = Config::new();
    println!("config, ``{:?}``", config);

    /* setup logging ------------------------- */
    let mut log_builder = Builder::from_default_env();
    log_builder.target(Target::Stdout);
    log_builder.init();
    let msg: String = format!(
        "\n\n-------\n`starting logchecker_project code at, ``{:?}``",
        local_time.to_rfc3339()
    );
    info!("{}", msg);

    /* load log_paths.json file (sync) ------- */
    let log_paths_obj: PathsSource = PathsSource::load_log_paths(&config.logs_json_file_path);
    debug!("log_paths_obj, ``{:?}``", log_paths_obj);

    /* get list of candidate files (async) --- */
    let dir_paths: Vec<String> = log_paths_obj.dir_paths;
    // debug!( "{}", format!("dir_paths, ``{:?}``", dir_paths) );
    // let z: () = log_paths_obj.dir_paths; // yields: found struct `Vec<String>`
    // for dir_path in dir_paths {  // dir_path is a String
    //     debug!("{}", format!("dir_path, ``{:?}``", dir_path));
    // }

    let candidates_from_dirs: Vec<String> = evaluate_dirs(dir_paths).await;
    debug!("candidates_from_dirs, ``{:?}``", candidates_from_dirs);

    /* process each candidate file (async) --- */

    /* massage output and email (sync) ------- */

    /* output elapsed time */
    let duration: Duration = start_time.elapsed();
    info!("elapsed-time, ``{:?}``", duration);
}
