extern crate ncurses;

use std::fs;


pub fn run(file_nm: String) {
    let contents = read_file(file_nm);
    ncurses::initscr();
    loop {
        draw(contents);
        let ch = get_user_input();
        println!("ch {}", ch);
        break;
    }
    ncurses::endwin();
}


fn read_file(file_nm: String) -> String {
    fs::read_to_string(file_nm)
        .expect("There was a problem reading this file.")
}


fn draw(contents: String) {
    ncurses::addstr(
        format!("Contents:\n\n{}", contents).as_str()
    );
}


fn get_user_input() -> i32 {
    ncurses::getch()
}

