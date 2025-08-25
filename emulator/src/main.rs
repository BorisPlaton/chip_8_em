use crate::chip::init_chip8;
use crate::frame::Frame;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use std::collections::HashMap;

mod chip;
mod frame;

fn main() {
    let program_path = "./roms/test-quirks.ch8".to_string();
    let mut chip8 = init_chip8(program_path);

    let scale = 10;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("CHIP-8", 64 * scale, 32 * scale)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.set_scale(scale as f32, scale as f32).unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_target(PixelFormatEnum::RGB24, 64, 32)
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut frame = Frame::default();
    let mut key_map = HashMap::new();
    key_map.insert(Keycode::NUM_1, 1);
    key_map.insert(Keycode::NUM_2, 2);
    key_map.insert(Keycode::NUM_3, 3);
    key_map.insert(Keycode::NUM_4, 0xC);

    key_map.insert(Keycode::Q, 4);
    key_map.insert(Keycode::W, 5);
    key_map.insert(Keycode::E, 6);
    key_map.insert(Keycode::R, 0xD);

    key_map.insert(Keycode::A, 7);
    key_map.insert(Keycode::S, 8);
    key_map.insert(Keycode::D, 9);
    key_map.insert(Keycode::F, 0xE);

    key_map.insert(Keycode::Z, 0xA);
    key_map.insert(Keycode::X, 0);
    key_map.insert(Keycode::C, 0xB);
    key_map.insert(Keycode::V, 0xF);

    chip8.run(|keyboard, display| {
        frame.update(display);

        texture.update(None, frame.pixels(), 64 * 3).unwrap();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => std::process::exit(0),
                Event::KeyDown { keycode, .. } => {
                    if let Some(&key) = key_map.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                        keyboard.press_key(key);
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    if let Some(&key) = key_map.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                        keyboard.release_key(key);
                    }
                }
                _ => { /* do nothing */ }
            }
        }
    })
}
