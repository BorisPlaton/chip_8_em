use chip8::chip::Chip8;
use chip8::platform::{ChipMode, Quirks};
use chip8::rom::Rom;
use std::collections::HashSet;

pub fn init_chip8<'a>(
    file: &'a str,
    mode: &'a ChipMode,
    quirks: &'a HashSet<Quirks>,
    ticks: u16,
    sleep: Option<u8>,
) -> Chip8<'a> {
    let rom = Rom::new(file);
    Chip8::new(rom, mode, quirks, ticks as u32, sleep)
}
