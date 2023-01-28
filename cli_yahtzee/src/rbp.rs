#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_macros)]

extern crate ncurses;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::io::Write;

// Seeds the pseudo-random number generator with unix time.
pub fn seed() {
    let d = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap();
    let _rng = StdRng::seed_from_u64(d.as_secs());
}

// Generates a pseudo-random number between x and y.
pub fn pseudo(x: i32, y: i32) -> i32 {
    return rand::thread_rng().gen_range(x..y + 1);
}

// Opens the virtual terminal.
pub fn vt_open() {
    ncurses::initscr();
    ncurses::raw();
    ncurses::scrollok(ncurses::stdscr(), true);
    ncurses::keypad(ncurses::stdscr(), true);
}

// Displays a message, then closes the virtual terminal on the next user key press.
pub fn vt_close(x: &str) {
    ncurses::addstr(x);
    ncurses::getch();
    ncurses::endwin();
}

// Hides the virtual cursor.
pub fn vt_cursor_off() {
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
}

// Clears the virtual terminal.
pub fn vt_cls() {
    ncurses::clear();
}

// Obtains an i32 integer from a virtual terminal key press.
pub fn vt_key_i32() -> i32 {
    let ch = ncurses::getch();
    ch
}

// Obtains a char from a virtual terminal key press.
pub fn vt_key_char() -> char {
    let ch = ncurses::getch();
    let ch_u8 = ch as u8;
    let ch_char = ch_u8 as char;
    ch_char
}

// Displays a slice in the virtual terminal.
pub fn vt_put_slice(x: &str) {
    ncurses::addstr(x);
    ncurses::refresh();
}

// Writes data to a file.
pub fn write_to_file(path: &str, data: &str) {
    std::fs::write(path, data).unwrap();
}

// Returns true if the file path exists.
pub fn file_exists(path: &str) -> bool {
    std::fs::metadata(path).is_ok()
}

// Appends data to a file.
pub fn append_to_file(path: &str, data: &str) {
    if file_exists(path) {
        let mut file = std::fs::OpenOptions::new().append(true).open(path).unwrap();
        file.write_all(data.as_bytes()).unwrap();
    } else {
        write_to_file(path, data);
    }
}
