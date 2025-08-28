use crate::chip::init_chip8;
use crate::devices::display::DisplayDevice;
use crate::devices::keyboard::KeyboardDevice;

mod chip;
mod devices;

fn main() {
    let sdl_context = sdl2::init().unwrap();

    let mut chip8 = init_chip8("./roms/slipperyslope.ch8".to_string());
    let mut display_device = DisplayDevice::new(&sdl_context, "CHIP-8", 64, 32, 10);
    let mut keyboard_device = KeyboardDevice::new(&sdl_context);

    chip8.run(|keyboard, display, st_register_val| {
        display_device.draw(display);

        // if st_register_val > 0 {
        //     device.resume();
        // } else {
        //     device.pause();
        // }

        keyboard_device
            .keys_state()
            .unwrap_or_else(|_| std::process::exit(0))
            .iter()
            .enumerate()
            .for_each(|(key, &is_pressed)| {
                if is_pressed {
                    keyboard.press_key(key as u8);
                } else {
                    keyboard.release_key(key as u8);
                }
            });
    })
}
