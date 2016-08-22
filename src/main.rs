extern crate sdl2;

use std::env;

mod filemap;
use filemap::FileMap;
use filemap::Character;

// fn print_file_map (map: &FileMap) {
//     for line in &map.lines {
//         for character in &line.characters {
//             match character {
//                 &Character::Whitespace => { print!(" ") },
//                 &Character::Normal => { print!("X") }
//             }
//         }
//         print!("\n")
//     }
// }

use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::render::Texture;

struct DisplayObject {
    texture: Texture,
    rect: Rect
}

fn render_with_sdl(maps: &Vec<FileMap>) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("maximap", 500, 500)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut renderer = window.renderer().present_vsync().build().unwrap();
    renderer.set_draw_color(Color::RGB(255, 255, 255));

    let mut x_offset = 0;
    let display_objs = maps.iter().map(|map| {
        let mut texture =
            renderer.create_texture_streaming(PixelFormatEnum::RGB24, map.width, map.height)
                .unwrap();
        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..map.height {
                for x in 0..map.width {
                    let offset = y as usize * pitch + x as usize * 3;
                    buffer[offset + 0] = 255;
                    buffer[offset + 1] = 255;
                    buffer[offset + 2] = 255;
                }
            }
            for (y, line) in map.lines.iter().enumerate() {
                for (x, character) in line.characters.iter().enumerate() {
                    let offset = y as usize * pitch + x as usize * 3;
                    match character {
                        &Character::Whitespace => { },
                        &Character::Normal => {
                            buffer[offset + 0] = 64;
                            buffer[offset + 1] = 64;
                            buffer[offset + 2] = 64;
                        }
                    }
                }
            }
        })
        .unwrap();

        let rect = Rect::new(x_offset as i32, 0, map.width, map.height);

        x_offset += map.width + 10;

        DisplayObject {
            texture: texture,
            rect: rect
        }
    }).collect::<Vec<_>>();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }
        {
            renderer.clear();
            for obj in &display_objs {
                renderer.copy(&obj.texture, None, Some(obj.rect));
            }
            renderer.present();
        }
    }
}

fn main() {
    let file_maps: Vec<FileMap> = env::args()
        .skip(1)
        .filter_map(|path| FileMap::from_path(&path))
        .collect::<Vec<FileMap>>();

    if !file_maps.is_empty() {
        render_with_sdl(&file_maps);
    }
}
