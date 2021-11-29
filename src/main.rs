#[macro_use]
extern crate log;

use chrono::Local;
use env_logger::{Builder, Target};
use tokio::time::{Duration, Instant};

use logchecker_project::Config;
use logchecker_project::PathsSource;


// use std::fs;
// use serde_json::{Value};


// use serde::Deserialize;




#[tokio::main]
async fn main() {
    /* start */
    let start_time = Instant::now();
    println!("start_time, ``{:?}``", start_time);
    let local_time = Local::now(); // used for logging

    /* load settings */
    let config: Config = Config::new();
    println!("config, ``{:?}``", config);

    /* setup logging */
    let mut log_builder = Builder::from_default_env();
    log_builder.target(Target::Stdout);
    log_builder.init();
    let msg: String = format!(
        "\n\n-------\n`starting logchecker_project code at, ``{:?}``",
        local_time.to_rfc3339()
    );
    info!("{}", msg);

    /* load log_paths.json file (sync) */
    // let log_paths_obj: String = load_log_paths( &config.logs_json_file_path );
    let log_paths_obj: PathsSource = PathsSource::load_log_paths( &config.logs_json_file_path );
    // let zz: () = log_paths_obj;
    debug!( "{}", format!("log_paths_obj, ``{:?}``", log_paths_obj) );

    /* get list of candidate files (async) */

    /* process each candidate file, saving output (async) */

    /* massage output and email (sync) */

    /* output elapsed time */
    let duration: Duration = start_time.elapsed();
    info!("{}", format!("elapsed-time, ``{:?}``", duration));
}



// fn load_log_paths( logs_json_file_path: &std::string::String ) -> String {
//     /*  Loads json list of paths into an iterable array.
//         Called by: main()  */

//     // --- read file ---
//     let jsn: String = fs::read_to_string( &logs_json_file_path ).unwrap_or_else(|error| {
//         panic!("Problem reading the json-file -- ``{:?}``", error);
//     });
//     // println!("\njsn, ``{:?}``", jsn);  // yields: jsn, ``"[\n  {\n    \"path\": \"/foo/the.log\"\n  },\n  {\n    \"path\": \"/path/to/logs/addto_refworks_logs/addto_refworks.log\"\n  },\n  {\n    \"path\": \"/path/to/logs/annex_counts_logs/annex_counts.log\"\n  }\n]\n"``
//     // let zz: () = jsn;  // yields: found struct `std::string::String`

//     // --- turn String into json-object ---
//     let ps: PathsSource = serde_json::from_str(&jsn).unwrap_or_else(|error| {
//         panic!("Problem converting the json-file to an object -- maybe invalid json? -- ``{:?}``", error);
//     });
//     debug!( "{}", format!("ps, ``{:?}``", ps) );
//     debug!( "{}", format!("ps.dir_paths, ``{:?}``", ps.dir_paths) );


//     return "foo".to_string();
// }


// #[derive(Deserialize, Debug)]
// pub struct PathsSource {
//     pub dir_paths: Vec<String>,
//     pub file_paths: Vec<String>,
// }

