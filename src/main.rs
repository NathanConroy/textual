use std::env;

fn main() {
    let config = parse_args();
    println!("{}", config.file_nm);
}


fn parse_args() -> textual::Config {
    // Returns the path to the file to edit.
    let args: Vec<String> = env::args().collect();
    textual::Config {
        file_nm: String::from(&args[1]),
    }
}
