use std::fmt::Display;

#[derive(PartialEq)]
pub enum ChipMode {
    Chip8,
    SuperChip,
}

impl Display for ChipMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChipMode::Chip8 => write!(f, "CHIP-8"),
            ChipMode::SuperChip => write!(f, "SUPER-CHIP"),
        }
    }
}
