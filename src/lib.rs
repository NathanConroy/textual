extern crate ncurses;

use std::fs;
use std::char;


pub fn run(file_nm: String) {
    let contents = read_file(file_nm);
    setup();
    loop {
        draw(&contents);
        let ch = get_user_input() as u32;
        match char::from_u32(ch) {
            Some('q') => {
                break;
            },
            _ => ()
        }
    }
    close();
}


fn setup() {
    ncurses::initscr();
    ncurses::keypad(ncurses::stdscr(), true);
    ncurses::raw();
    ncurses::cbreak();
}


fn close() {
    ncurses::endwin();
}


fn read_file(file_nm: String) -> String {
    fs::read_to_string(file_nm)
        .expect("There was a problem reading this file.")
}


fn draw(contents: &String) {
    ncurses::clear();
    ncurses::addstr(
        format!("{}\n\nEnter 'q' to quit: ", contents).as_str()
    );
}


fn get_user_input() -> i32 {
    ncurses::getch()
}

