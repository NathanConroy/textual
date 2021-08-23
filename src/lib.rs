use std::fs;


pub fn run(file_nm: String) {
    let contents = read_file(file_nm);
    loop {
        draw(contents);
        break;
    }
}

fn read_file(file_nm: String) -> String {
    println!("Opening {}.\n", file_nm);
    fs::read_to_string(file_nm)
        .expect("There was a problem reading this file.")
}

fn draw(contents: String) {
    println!("Contents:\n\n{}", contents);
}

