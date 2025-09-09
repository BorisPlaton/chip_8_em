use crate::chip::init_chip8;
use crate::cli::parser::EmulatorConfig;
use crate::devices::audio::AudioDevice;
use crate::devices::display::DisplayDevice;
use crate::devices::keyboard::KeyboardDevice;
use chip8::display::Display;

mod chip;
mod cli;
mod devices;

fn main() {
    let sdl_context = sdl2::init().unwrap();

    let config = EmulatorConfig::new();
    let mut chip8 = init_chip8(&config);

    let audio_device = AudioDevice::new(&sdl_context);
    let mut keyboard_device = KeyboardDevice::new(&sdl_context);
    let mut display_device = DisplayDevice::new(
        &sdl_context,
        "CHIP-8",
        Display::EXTENDED_WIDTH as u32,
        Display::EXTENDED_HEIGHT as u32,
        config.scale as u32,
    );

    chip8.run(|keyboard, display, st_register_val| {
        display_device.draw(display);
        audio_device.play_sound(st_register_val);
        keyboard_device
            .keys_state()
            .iter()
            .enumerate()
            .for_each(|(key, &is_pressed)| {
                if is_pressed {
                    keyboard.press_key(key as u8);
                } else {
                    keyboard.release_key(key as u8);
                }
            });
    });
}
