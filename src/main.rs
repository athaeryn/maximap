extern crate sdl2;

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

use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    let path_string = match env::args().nth(1) {
        Some(arg) => arg,
        None => panic!("USAGE: maximap FILENAME")
    };

    if let Ok(map) = FileMap::from_path(&path_string) {
        println!("{}", map);

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("maximap", map.width, map.height)
            .position_centered()
            .build()
            .unwrap();
        let mut renderer = window.renderer().build().unwrap();
        let mut texture = renderer.create_texture_streaming(PixelFormatEnum::RGB24, map.width, map.height).unwrap();
        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for (y, line) in map.lines.iter().enumerate() {
                for (x, character) in line.characters.iter().enumerate() {
                    let offset = y as usize * pitch + x as usize * 3;
                    let value = match character {
                        &Character::Whitespace => 0,
                        &Character::Normal => 255
                    };
                    buffer[offset + 0] = value;
                    buffer[offset + 1] = value;
                    buffer[offset + 2] = value;
                }
            }
        }).unwrap();

        renderer.clear();
        renderer.copy(&texture, None, Some(Rect::new(0, 0, map.width, map.height)));
        renderer.present();

        let mut event_pump = sdl_context.event_pump().unwrap();

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => { break 'running },
                    _ => { }
                }
            }
        }
    }
}

