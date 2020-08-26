use std::env;
use std::path::PathBuf;

mod cli;


fn main() {
    let matches = cli::build_cli().get_matches();
    let root_dir = match matches.value_of("root").unwrap() {
        "." => env::current_dir().unwrap(),
        path => PathBuf::from(path),
    };

}
