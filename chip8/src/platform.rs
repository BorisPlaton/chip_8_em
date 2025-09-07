use std::fmt::Display;

#[derive(PartialEq)]
pub enum ChipMode {
    Chip8,
    SuperChip,
}

#[derive(Hash, Eq, PartialEq)]
pub enum Quirks {
    /// For `FX55` and `FX65` instructions.
    ///
    /// CHIP-8 interpreter incremented the `I` register while it worked.
    /// Each time it stored or loaded one register, it incremented `I`.
    /// After the instruction was finished, I would end up being set to
    /// the new value `I` + `X` + 1.
    ///
    /// Modern interpreters (starting with CHIP48 and SUPER-CHIP in the
    /// early 90s) used a temporary variable for indexing, so when the
    /// instruction was finished, `I` would still hold the same value
    /// as it did before.
    IRegisterIncrementedWithX,

    /// For `BNNN` instruction.
    ///
    /// In the original COSMAC VIP interpreter, this instruction jumped
    /// to the address NNN plus the value in the register V0.
    ///
    /// Starting with CHIP-48 and SUPER-CHIP, it was (probably unintentionally)
    /// changed to work as `BXNN`: It will jump to the address `XNN`,
    /// plus the value in the register `VX`.
    JumpWithX,

    /// For `8XY6` and `8XYE` instructions.
    ///
    /// In the CHIP-8 interpreter, this instruction did the following:
    /// It put the value of `VY` into `VX`, and then shifted the value
    /// in `VX` 1 bit to the right (`8XY6`) or left (`8XYE`). `VY` was
    /// not affected, but the flag register `VF` would be set to the bit
    /// that was shifted out.
    ///
    /// However, starting with CHIP-48 and SUPER-CHIP in the early 1990s,
    /// these instructions were changed so that they shifted `VX` in place,
    /// and ignored the `VY` completely.
    ShiftIgnoreVY,

    /// For `8XY1`, `8XY2` and `8XY3` instructions.
    ///
    /// The AND, OR and XOR opcodes reset the flags register to zero in the end.
    BinaryOpResetVF,
}

impl Display for ChipMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChipMode::Chip8 => write!(f, "CHIP-8"),
            ChipMode::SuperChip => write!(f, "SUPER-CHIP"),
        }
    }
}
