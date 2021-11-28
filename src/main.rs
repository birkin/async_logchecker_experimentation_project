#[macro_use]
extern crate log;

use chrono::Local;
use env_logger::{Builder, Target};
use tokio::time::{Duration, Instant};

use logchecker_project::Config;


use std::fs;
use serde_json::{Value};



#[tokio::main]
async fn main() {
    /* start */
    let start_time = Instant::now();
    println!("start_time, ``{:?}``", start_time);
    let local_time = Local::now(); // used for logging

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
    info!("{}", msg);

    /* load log_paths.json file (sync) */
    let log_paths_obj: std::vec::Vec<serde_json::value::Value> = load_log_paths( &config.logs_json_file_path );

    /* get list of candidate files (async) */

    /* process each candidate file, saving output (async) */

    /* massage output and email (sync) */

    /* output elapsed time */
    let duration: Duration = start_time.elapsed();
    info!("{}", format!("elapsed-time, ``{:?}``", duration));
}



fn load_log_paths( logs_json_file_path: &std::string::String ) -> std::vec::Vec<serde_json::value::Value> {
    /*  Loads json list of paths into an iterable array.
        Called by: main()  */

    // --- read file ---
    let jsn: String = fs::read_to_string( &logs_json_file_path ).unwrap_or_else(|error| {
        panic!("Problem reading the json-file -- ``{:?}``", error);
    });
    // println!("\njsn, ``{:?}``", jsn);  // yields: jsn, ``"[\n  {\n    \"path\": \"/foo/the.log\"\n  },\n  {\n    \"path\": \"/path/to/logs/addto_refworks_logs/addto_refworks.log\"\n  },\n  {\n    \"path\": \"/path/to/logs/annex_counts_logs/annex_counts.log\"\n  }\n]\n"``
    // let zz: () = jsn;  // yields: found struct `std::string::String`

    // --- turn String into json-object ---
    let paths_obj: Value = serde_json::from_str(&jsn).unwrap_or_else(|error| {
        panic!("Problem converting the json-file to an object -- maybe invalid json? -- ``{:?}``", error);
    });
    println!("\npaths_obj, ``{:?}``", paths_obj);  // yields: paths_obj, ``Object({"dir_paths": Array([String("foo1"), String("foo2")]), "file_paths": Array([String("bar1"), String("bar2")])})``
    // println!("\npaths_obj, ``{:?}``", paths_obj); // yields: paths_obj, ``Array([Object({"path": String("/foo/the.log")}), Object({"path": String("/path/to/logs/addto_refworks_logs/addto_refworks.log")}), Object({"path": String("/path/to/logs/annex_counts_logs/annex_counts.log")})])``
    // let zz: () = paths_obj;  // yields: found enum `Value`
    // let zz: () = paths_obj;  // yields: found enum `serde_json::value::Value`

    let dir_paths: &Value = &paths_obj["dir_paths"];
    // let zz: () = dir_paths;  // yields: found `&Value`
    println!("dir_paths, ``{:?}``", dir_paths);

    let dir_paths_array = dir_paths.as_array().unwrap_or_else(|| {
        panic!("Problem handling dir_paths_array.");
    });
    // let zz: () = dir_paths_array;  // yields: found `&Vec<Value>`
    debug!( "{}", format!("dir_paths_array, ``{:?}``", dir_paths_array) );

    let real_dir_paths_array: std::vec::Vec<serde_json::value::Value> = dir_paths_array.to_vec();
    // let zz: () = real_dir_paths_array;  // yields: found struct `Vec`
    debug!( "{}", format!("real_dir_paths_array, ``{:?}``", real_dir_paths_array) );







    return real_dir_paths_array;
}
