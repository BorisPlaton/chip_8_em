use chip8::chip::Chip8;
use chip8::platform::{ChipMode, Quirks};
use chip8::rom::Rom;
use std::collections::HashSet;

pub fn init_chip8<'a>(
    program_path: String,
    chip_mode: &'a ChipMode,
    quirks: &'a HashSet<Quirks>,
) -> Chip8<'a> {
    let rom = Rom::new(program_path);
    Chip8::new(rom, chip_mode, quirks)
}
