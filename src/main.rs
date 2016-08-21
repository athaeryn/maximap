use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[allow(dead_code)]
enum Character {
    Whitespace,
    Normal
    // TODO: Comment
}

impl Character {
    fn from_char(ch: char) -> Character {
        if ch.is_whitespace() {
            Character::Whitespace
        } else {
            Character::Normal
        }
    }
}

struct Line {
    characters: Vec<Character>,
    width: u32
}

impl Line {
    fn from_string (string: &str) -> Line {
        let characters: Vec<Character> = string
            .chars()
            .map(Character::from_char)
            .collect();

        let width: u32 = characters.len() as u32;

        Line {
            characters: characters,
            width: width
        }
    }
}

struct FileMap {
    lines: Vec<Line>,
    height: u32,
    width: u32,
    filename: String
}

use std::cmp;

impl FileMap {
    fn from_path(path_string: &str) -> Result<FileMap, &str> {
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

        let lines = contents
            .split("\n")
            .map(|string| Line::from_string(string))
            .collect::<Vec<Line>>();

        let width = lines
            .iter()
            .fold(0, |max, line| cmp::max(line.width, max));

        // TODO: why is the `- 1` necessary?
        let height = (lines.len() as u32) - 1;

        Ok(FileMap {
            lines: lines,
            height: height,
            width: width,
            filename: path_string.to_string()
        })
    }
}

use std::fmt;
impl fmt::Display for FileMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:<{},{}>", self.filename, self.width, self.height)
    }
}

fn print_file_map (map: &FileMap) {
    for line in &map.lines {
        for character in &line.characters {
            match character {
                &Character::Whitespace => { print!(" ") },
                &Character::Normal => { print!("X") }
            }
        }
        print!("\n")
    }
}

fn main() {
    let path_string = match env::args().nth(1) {
        Some(arg) => arg,
        None => panic!("USAGE: maximap FILENAME")
    };

    if let Ok(map) = FileMap::from_path(&path_string) {
        println!("{}", map);
        print_file_map(&map);
    }
}
