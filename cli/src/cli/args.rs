use clap::builder::PossibleValue;
use clap::{Parser, ValueEnum};
use clap_num::maybe_hex;

#[derive(Parser)]
pub struct Args {
    /// Path to CHIP program file.
    pub file: String,

    /// The CHIP platform to use.
    #[arg(short, long, value_enum, default_value_t = Platform::Chip8)]
    pub platform: Platform,

    /// Quirk for FX55 and FX65 instructions.
    ///
    /// CHIP-8 interpreter incremented the I register while it worked.
    ///
    /// Modern interpreters when the instruction was finished, the I
    /// would still hold the same value as it did before.
    ///
    /// Specifying this flag will enable CHIP-8 behaviour.
    #[arg(short, long)]
    pub load_increment_i_with_x_quirk: bool,

    /// Quirk for BNNN instruction.
    ///
    /// CHIP-8 interpreter jumped to the address NNN plus the value in
    /// the register V0.
    ///
    /// Starting with CHIP-48 and SUPER-CHIP It will jump to the address
    /// XNN, plus the value in the register VX.
    ///
    /// Specifying this flag will enable modern behaviour.
    #[arg(short, long)]
    pub jump_using_x_quirk: bool,

    /// Quirk for 8XY6 and 8XYE instructions.
    ///
    /// In the CHIP-8 interpreter, the instruction puts the value of VY
    /// into VX.
    ///
    /// Starting with CHIP-48 and SUPER-CHIP, the instruction shifted VX
    /// in place and ignored the VY completely.
    ///
    /// Specifying this flag will enable modern behaviour.
    #[arg(short, long)]
    pub shift_ignore_vy_quirk: bool,

    /// Quirk for 8XY1, 8XY2 and 8XY3 instructions.
    ///
    /// The AND, OR and XOR opcodes reset the VF register to zero in
    /// the end.
    ///
    /// Specifying this flag will reset VF for 8XY1, 8XY2 and 8XY3 instructions.
    #[arg(short, long)]
    pub binary_op_reset_vf_quirk: bool,

    /// Wraps pixels instead of clipping them.
    ///
    /// When this quirk is enabled, sprites get rendered at the coordinates on
    /// the other side of the screen.
    #[arg(short, long)]
    pub wrap_instead_of_clipping_quirk: bool,

    /// Scale of the emulator window.
    #[arg(long, default_value_t = 7, value_parser = clap::value_parser!(u8).range(..=13))]
    pub scale: u8,

    /// How many instructions executed per 1 video frame.
    ///
    /// Lowering this value, may lead to freezes.
    #[arg(short, long, default_value_t = 1000)]
    pub instructions_per_frame: u16,

    /// Program will wait this amount of microseconds after each instruction.
    ///
    /// Use this if the program is very fast and you want to slow down it.
    #[arg(long, value_parser = clap::value_parser!(u8))]
    pub sleep: Option<u8>,

    /// Set color in hex for disabled pixels.
    #[arg(long, default_value = "0x000000", value_parser = maybe_hex::<u32>, value_name = "DISABLED COLOR")]
    pub set_disabled_color: u32,

    /// Set color in hex for enabled pixels on the first plane.
    #[arg(long, default_value = "0xFF0000", value_parser = maybe_hex::<u32>, value_name = "FIRST PLANE VALUE")]
    pub set_first_plane_color: u32,

    /// Set color in hex for enabled pixels on the second plane.
    #[arg(long, default_value = "0x00FF00", value_parser = maybe_hex::<u32>, value_name = "SECOND PLANE VALUE")]
    pub set_second_plane_color: u32,

    /// Set color in hex for enabled pixels on the first and second plane.
    #[arg(long, default_value = "0x0000FF", value_parser = maybe_hex::<u32>, value_name = "BOTH PLANE VALUE")]
    pub set_both_plane_color: u32,
}

#[derive(Clone)]
pub enum Platform {
    Chip8,
    SuperChip,
    XOChip,
}

impl ValueEnum for Platform {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Chip8, Self::SuperChip, Self::XOChip]
    }

    fn from_str(input: &str, _ignore_case: bool) -> Result<Self, String> {
        match input.to_lowercase().as_str() {
            "chip8" => Ok(Self::Chip8),
            "superchip" => Ok(Self::SuperChip),
            "xochip" => Ok(Self::XOChip),
            _ => Err(format!("Invalid platform: {}", input)),
        }
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            Self::Chip8 => {
                Some(PossibleValue::new("chip8").help("Program will run only CHIP-8 instructions."))
            }
            Self::SuperChip => Some(
                PossibleValue::new("schip")
                    .help("Program will run only CHIP-8 + SuperChip instructions."),
            ),
            Self::XOChip => Some(
                PossibleValue::new("xochip")
                    .help("Program will run only CHIP-8 + SuperChip + XO-Chip instructions."),
            ),
        }
    }
}
