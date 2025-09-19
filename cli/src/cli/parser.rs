use crate::cli::args::{Args, Platform};
use chip8::display::Color;
use chip8::platform::{ChipMode, Quirks};
use clap::Parser;
use std::collections::{HashMap, HashSet};

pub struct EmulatorConfig {
    pub file: String,
    pub quirks: HashSet<Quirks>,
    pub mode: ChipMode,
    pub scale: u8,
    pub ticks: u16,
    pub sleep: Option<u8>,
    pub palette: HashMap<Color, (u8, u8, u8)>,
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
            palette: HashMap::from([
                (Color::Disabled, {
                    let red = (args.set_disabled_color >> 16) as u8;
                    let green = (args.set_disabled_color >> 8) as u8;
                    let blue = args.set_disabled_color as u8;
                    (red, green, blue)
                }),
                (Color::OnlyFirstPlane, {
                    let red = (args.set_first_plane_color >> 16) as u8;
                    let green = (args.set_first_plane_color >> 8) as u8;
                    let blue = args.set_first_plane_color as u8;
                    (red, green, blue)
                }),
                (Color::OnlySecondPlane, {
                    let red = (args.set_second_plane_color >> 16) as u8;
                    let green = (args.set_second_plane_color >> 8) as u8;
                    let blue = args.set_second_plane_color as u8;
                    (red, green, blue)
                }),
                (Color::Both, {
                    let red = (args.set_both_plane_color >> 16) as u8;
                    let green = (args.set_both_plane_color >> 8) as u8;
                    let blue = args.set_both_plane_color as u8;
                    (red, green, blue)
                }),
            ]),
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
