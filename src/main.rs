use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = textual::Config::new(args);
    println!("{}", config.file_nm);
}
