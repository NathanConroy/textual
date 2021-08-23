use std::env;
use std::process;

mod config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = config::Config::new(args).unwrap_or_else(|err_msg| {
        println!("{}", err_msg);
        process::exit(1);
    });
    textual::run(config.file_nm);
}
