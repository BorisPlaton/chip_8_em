use crate::memory::Memory;
use crate::stack::Stack;
use std::collections::HashMap;

pub struct Chip8 {
    memory: Memory,
    stack: Stack,
    /// General purpose registers.
    registers: HashMap<u8, u8>,
    /// I register is generally used to store memory addresses, so only
    /// the lowest (rightmost) 12 bits are usually used.
    i_register: u16,
    /// Delay timer register.
    dt_register: u8,
    /// Sound timer register.
    st_register: u8,
    /// PC is used to store the currently executing address.
    program_counter: u16,
}

impl Chip8 {
    pub fn new(memory: Memory) -> Chip8 {
        Chip8 {
            memory,
            stack: Stack::default(),
            i_register: 0,
            dt_register: 0,
            st_register: 0,
            program_counter: Memory::PROGRAM_ADDR_START,
            registers: {
                let mut registers = HashMap::with_capacity(0xF);
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
                registers
            },
        }
    }

    pub fn run(&mut self) {
        todo!()
    }
}
