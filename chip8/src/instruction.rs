use std::collections::HashMap;

/// Represents an CHIP-8 instruction, for instance:
/// * 00E0 - CLS (Clear the display)
/// * 00EE - RET (Return from a subroutine)
/// * etc.
///
/// All instructions are 2 bytes long. So, `0x00E0` - will be represented as:
///
/// `opcode` `1, 2, 3 parameter`
///    |        |
///  ++++ ++++++++++++++
/// `0000_0000_1110_0000`
pub struct Instruction {
    /// The initial form of received instruction.
    value: u16,
    /// The opcode contains the first nibble of instruction.
    opcode: u8,
    /// The remaining nibbles of instruction.
    parameters: (u8, u8, u8),
}

impl Instruction {
    pub fn new(value: u16) -> Instruction {
        let bytes: [u8; 2] = value.to_be_bytes().try_into().unwrap();
        let opcode = bytes[0] >> 4;
        let first_nibble = bytes[0] & 0x0F;
        let second_nibble = bytes[1] >> 4;
        let third_nibble = bytes[1] & 0x0F;

        let mut registers = HashMap::with_capacity(16);
        registers.insert(0x0, 0);
        registers.insert(0x1, 0);
        registers.insert(0x2, 0);
        registers.insert(0x3, 0);
        registers.insert(0x4, 0);
        registers.insert(0x5, 0);
        registers.insert(0x6, 0);
        registers.insert(0x7, 0);
        registers.insert(0x8, 0);
        registers.insert(0x9, 0);
        registers.insert(0xA, 0);
        registers.insert(0xB, 0);
        registers.insert(0xC, 0);
        registers.insert(0xD, 0);
        registers.insert(0xE, 0);
        registers.insert(0xF, 0);

        Instruction {
            value,
            opcode,
            parameters: (first_nibble, second_nibble, third_nibble),
        }
    }

    pub fn opcode(&self) -> u8 {
        self.opcode
    }

    pub fn parameters(&self) -> (u8, u8, u8) {
        self.parameters
    }

    pub fn value(&self) -> u16 {
        self.value
    }
}
