extern crate sdl2;

use std::env;

mod filemap;
use filemap::FileMap;
use filemap::Character;

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
        // print_file_map(&map);

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

