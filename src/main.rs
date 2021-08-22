use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = textual::Config::new(args).unwrap_or_else(|err_msg| {
        println!("{}", err_msg);
        process::exit(1);
    });
    println!("{}", config.file_nm);
}
