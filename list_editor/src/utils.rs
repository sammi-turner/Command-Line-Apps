extern crate ncurses;
use std::io::Read;
use std::io::Write;

// A macro to initialise a vector of strings.
macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

// Returns the nth char (zero indexed) from a slice.
pub fn nth_char(x: &str, n: usize) -> char {
    x.chars().nth(n).unwrap()
}

// Checks if a slice represents a positive integer.
pub fn is_pos_int(x: &str) -> bool {
    x.parse::<u8>().is_ok() && nth_char(x, 0) != '0'
}

// Converts a slice to a usize integer.
pub fn to_usize(x: &str) -> usize {
    x.parse::<usize>().unwrap()
}

// Counts the number of chars in a slice.
pub fn char_count(x: &str) -> usize {
    x.chars().count()
}

// Counts the number of lines in a slice.
pub fn line_count(x: &str) -> usize {
    if x == "" {
        return 0;
    }
    let mut count: usize = 0;
    let word_vec = x.split("\n");
    for _ in word_vec {
        count += 1;
    }
    count
}

// Returns the nth line (zero indexed) from a slice.
pub fn nth_line(x: &str, y: usize) -> &str {
    if x == "" || y >= line_count(x) {
        return "";
    }
    let line_vec = x.split("\n");
    let mut result = "";
    let mut count: usize = 0;
    for r in line_vec {
        if y == count {
            result = r;
        }
        count += 1;
    }
    result
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

// Reads from a file.
pub fn read_from_file(path: &str) -> String {
    let mut f = std::fs::File::open(path).unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    buffer
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

// Hides user keypresses.
pub fn vt_keypress_off() {
    ncurses::noecho();
}

// Displays user keypresses.
pub fn vt_keypress_on() {
    ncurses::echo();
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

// Obtains user input as a string with no more than x chars.
pub fn vt_input(x: i32) -> String {
    vt_keypress_on();
    let mut y = String::new();
    ncurses::getnstr(&mut y, x);
    y
}

// Displays a slice in the virtual terminal.
pub fn vt_put_slice(x: &str) {
    ncurses::addstr(x);
    ncurses::refresh();
}

// A helper function called by vt_menu.
pub fn vt_render_menu(menu: &mut Vec<String>, size: usize, count: usize) {
    vt_cls();
    vt_put_slice("\n     ");
    vt_put_slice(&mut menu[0]);
    vt_put_slice("\n\n");

    let mut n: usize = 1;
    while n < size {
        if n == count {
            vt_put_slice("   > ");
            vt_put_slice(&mut menu[n]);
            vt_put_slice("\n");
        } else {
            vt_put_slice("     ");
            vt_put_slice(&mut menu[n]);
            vt_put_slice("\n");
        }
        n += 1;
    }
}

// Returns a usize integer based on the user's selection from a menu.
pub fn vt_menu(menu: &mut Vec<String>) -> usize {
    vt_keypress_off();
    vt_cursor_off();

    let mut value: usize = 1;
    let size = menu.len();

    loop {
        vt_render_menu(menu, size, value);
        let key_press = ncurses::getch();

        if key_press == ncurses::KEY_DOWN {
            value += 1;
            if value == size {
                value = 1;
            }
            vt_render_menu(menu, size, value);
        }

        if key_press == ncurses::KEY_UP {
            value -= 1;
            if value == 0 {
                value = size - 1;
            }
            vt_render_menu(menu, size, value);
        }

        if key_press == ncurses::KEY_RIGHT {
            break;
        }

        if key_press == ncurses::KEY_ENTER {
            break;
        }

        if key_press == 10 {
            break;
        }
    }

    value
}

// A helper function called by vt_edit_prompt.
pub fn vt_render_prompt(prompt: &str, buffer: &mut String, pos: usize) {
    vt_cls();
    let mut s = String::from(prompt);
    for i in 0..pos {
        let ch = nth_char(&buffer, i);
        s.push(ch);
    }
    vt_put_slice(&s);
}

// Displays a prompt to the user with an existing buffer, which can be edited to return a new buffer.
pub fn vt_edit_prompt(prompt: &str, buffer: &mut String, max: usize) -> String {
    let mut exit = false;
    let mut result = buffer.clone();
    let mut pos = char_count(&buffer);
    let mut res = pos.clone();

    vt_render_prompt(&prompt, &mut result, pos);

    while !exit && pos < max {
        let ch = ncurses::getch();

        if ch == ncurses::KEY_LEFT && pos > 0 {
            pos -= 1;
        }
        if ch == ncurses::KEY_RIGHT && res > pos {
            pos += 1;
        }

        if ch == 127 && pos > 0 {
            pos -= 1;
            res -= 1;
            let _ = result.pop();
        } else if ch > 31 && ch < 127 {
            let ch_u8 = ch as u8;
            let ch_char = ch_u8 as char;

            if pos < res {
                result.replace_range(pos..pos + 1, &ch_char.to_string());
                pos += 1;
            } else {
                result.push(ch_char);
                pos += 1;
                res += 1;
            }
        } else if ch == 10 {
            exit = true;
        }
        vt_render_prompt(&prompt, &mut result, pos);
    }

    return result;
}
