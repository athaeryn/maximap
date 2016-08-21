use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[allow(dead_code)]
enum Character {
    Whitespace,
    Newline,
    Normal
    // TODO: Comment
}

impl Character {
    fn from_char(ch: char) -> Character {
        if ch == '\n' {
            Character::Newline
        } else if ch.is_whitespace() {
            Character::Whitespace
        } else {
            Character::Normal
        }
    }
}

#[allow(dead_code)]
struct FileMap {
    lines: Vec<Vec<Character>>
}


use std::env;

fn main() {
    let path_string = match env::args().nth(1) {
        Some(arg) => arg,
        None => panic!("USAGE: maximap FILENAME")
    };
    let path = Path::new(&path_string);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file
    };

    let mut contents = String::new();

    // TODO: better error handling
    file.read_to_string(&mut contents).unwrap();

    for ch in contents.chars() {
        match Character::from_char(ch) {
            Character::Normal => { print!("X") },
            Character::Newline => { print!("{}", '\n') },
            Character::Whitespace => { print!(" ") }
        }
    }
}
