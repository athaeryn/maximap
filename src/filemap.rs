use std::cmp;
use std::fmt;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub enum Character {
    Whitespace,
    Normal,
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

pub struct Line {
    pub characters: Vec<Character>,
    width: u32,
}

impl Line {
    fn from_string(string: &str) -> Line {
        let characters: Vec<Character> = string.chars()
            .map(Character::from_char)
            .collect();

        let width: u32 = characters.len() as u32;

        Line {
            characters: characters,
            width: width,
        }
    }
}

pub struct FileMap {
    pub lines: Vec<Line>,
    pub height: u32,
    pub width: u32,
    pub filename: String,
}

impl FileMap {
    pub fn from_path(path_string: &str) -> Option<FileMap> {
        let path = Path::new(&path_string);
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why.description()),
            Ok(file) => file,
        };

        let mut contents = String::new();

        // TODO: better error handling
        file.read_to_string(&mut contents).unwrap();

        let lines = contents.split("\n")
            .map(|string| Line::from_string(string))
            .collect::<Vec<Line>>();

        Some(FileMap {
            // TODO: why is the `- 1` necessary?
            height: (lines.len() as u32) - 1,
            width: lines.iter().fold(0, |max, line| cmp::max(line.width, max)),
            filename: path_string.to_string(),
            lines: lines,
        })
    }
}

impl fmt::Display for FileMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:<{},{}>", self.filename, self.width, self.height)
    }
}
