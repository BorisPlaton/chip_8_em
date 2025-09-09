use crate::cli::parser::EmulatorConfig;
use chip8::chip::Chip8;
use chip8::rom::Rom;

pub fn init_chip8(chip_config: &EmulatorConfig) -> Chip8 {
    let rom = Rom::new(&chip_config.file);
    Chip8::new(
        rom,
        &chip_config.mode,
        &chip_config.quirks,
        chip_config.ticks,
        chip_config.sleep,
    )
}
