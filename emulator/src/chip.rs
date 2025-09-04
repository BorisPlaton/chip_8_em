use chip8::chip::Chip8;
use chip8::modes::ChipMode;
use chip8::rom::Rom;

pub fn init_chip8(program_path: String, chip_mode: &ChipMode) -> Chip8 {
    let rom = Rom::new(program_path);
    Chip8::new(rom, chip_mode)
}
