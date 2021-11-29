#[macro_use]
extern crate log;

use std::fs;

use serde::Deserialize;
use std::env;

/* ------- Config ------- */

#[derive(Deserialize, Debug)]
pub struct Config {
    log_level: String,
    pub logs_json_file_path: String,
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

/* ------- PathsSource ------- */



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



#[derive(Deserialize, Debug)]
pub struct PathsSource {
    pub dir_paths: Vec<String>,
    pub file_paths: Vec<String>,
}

impl PathsSource {
    pub fn load_log_paths( logs_json_file_path: &std::string::String ) -> String {
        /*  Loads json list of paths into an iterable array.
            Called by: main()  */

        // --- read file ---
        let jsn: String = fs::read_to_string( &logs_json_file_path ).unwrap_or_else(|error| {
            panic!("Problem reading the json-file -- ``{:?}``", error);
        });
        // println!("\njsn, ``{:?}``", jsn);  // yields: jsn, ``"[\n  {\n    \"path\": \"/foo/the.log\"\n  },\n  {\n    \"path\": \"/path/to/logs/addto_refworks_logs/addto_refworks.log\"\n  },\n  {\n    \"path\": \"/path/to/logs/annex_counts_logs/annex_counts.log\"\n  }\n]\n"``
        // let zz: () = jsn;  // yields: found struct `std::string::String`

        // --- turn String into json-object ---
        let ps: PathsSource = serde_json::from_str(&jsn).unwrap_or_else(|error| {
            panic!("Problem converting the json-file to an object -- maybe invalid json? -- ``{:?}``", error);
        });
        debug!( "{}", format!("ps, ``{:?}``", ps) );
        debug!( "{}", format!("ps.dir_paths, ``{:?}``", ps.dir_paths) );


        return "foo".to_string();
        // return ps;
    }


}

