/// Represents an CHIP-8 instruction, for instance:
/// * 00E0 - CLS (Clear the display)
/// * 00EE - RET (Return from a subroutine)
/// * etc.
///
/// All instructions are 2 bytes long. So, `0x00E0` - will be represented as:
///
///  `opcode`  remaining parameters
///      |        |
///    ++++ ++++++++++++++
///   `0000_0000_1110_0000`
#[derive(Debug)]
pub struct Instruction {
    /// The initial form of received instruction.
    value: u16,
}

impl Instruction {
    pub fn new(value: u16) -> Instruction {
        Instruction { value }
    }

    pub fn nibbles(&self) -> (u8, u8, u8, u8) {
        let first_nibble = ((self.value & 0xF000) >> 12) as u8;
        let second_nibble = ((self.value & 0x0F00) >> 8) as u8;
        let third_nibble = ((self.value & 0x00F0) >> 4) as u8;
        let fourth_nibble = (self.value & 0xF) as u8;
        (first_nibble, second_nibble, third_nibble, fourth_nibble)
    }

    /// nnn or addr - A 12-bit value, the lowest 12 bits of the instruction
    pub fn nnn(&self) -> u16 {
        self.value & 0x0FFF
    }

    /// n or nibble - A 4-bit value, the lowest 4 bits of the instruction
    pub fn n(&self) -> u8 {
        self.value as u8 & 0xF
    }

    /// x - A 4-bit value, the lower 4 bits of the high byte of the instruction
    pub fn x(&self) -> u8 {
        ((self.value & 0x0F00) >> 8) as u8
    }

    /// y - A 4-bit value, the upper 4 bits of the low byte of the instruction
    pub fn y(&self) -> u8 {
        ((self.value & 0x00F0) >> 4) as u8
    }

    /// kk or byte - An 8-bit value, the lowest 8 bits of the instruction
    pub fn kk(&self) -> u8 {
        self.value as u8
    }
}
