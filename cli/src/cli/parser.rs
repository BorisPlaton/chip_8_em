use crate::cli::args::{Args, Platform};
use chip8::platform::{ChipMode, Quirks};
use clap::Parser;
use std::collections::HashSet;

pub struct EmulatorConfig {
    pub file: String,
    pub quirks: HashSet<Quirks>,
    pub mode: ChipMode,
    pub scale: u8,
    pub ticks: u16,
    pub sleep: Option<u8>,
}

impl EmulatorConfig {
    pub fn new() -> EmulatorConfig {
        let args = Args::parse();
        let mut quirks = HashSet::new();

        if args.load_increment_i_with_x_quirk {
            quirks.insert(Quirks::IRegisterIncrementedWithX);
        }
        if args.jump_using_x_quirk {
            quirks.insert(Quirks::JumpWithX);
        }
        if args.shift_ignore_vy_quirk {
            quirks.insert(Quirks::ShiftIgnoreVY);
        }
        if args.binary_op_reset_vf_quirk {
            quirks.insert(Quirks::BinaryOpResetVF);
        }
        if args.wrap_instead_of_clipping_quirk {
            quirks.insert(Quirks::WrapsInsteadClipping);
        }

        EmulatorConfig {
            file: args.file,
            mode: Self::get_chip_mode(&args.platform),
            scale: args.scale,
            ticks: args.instructions_per_frame,
            sleep: args.sleep,
            quirks,
        }
    }

    fn get_chip_mode(platform: &Platform) -> ChipMode {
        match platform {
            Platform::Chip8 => ChipMode::Chip8,
            Platform::SuperChip => ChipMode::SuperChip,
            Platform::XOChip => ChipMode::XOChip,
        }
    }
}
