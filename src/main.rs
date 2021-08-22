use std::env;

fn main() {
    println!("{}", parse_args());
}


fn parse_args() -> String {
    // Returns the path to the file to edit.
    let args: Vec<String> = env::args().collect();
    return String::from(&args[1]);
}
