use crate::instruction::Instruction;
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
        loop {
            let instruction = self.next_instruction();
            match (
                instruction.opcode(),
                instruction.value(),
                instruction.parameters(),
            ) {
                (_, 0x00E0, _) => self.cls(instruction),
                (_, 0x00EE, _) => self.ret(instruction),
                (0, _, _) => self.sys(instruction),
                (1, _, _) => self.jp_addr(instruction),
                (2, _, _) => self.call(instruction),
                (3, _, _) => self.se_byte(instruction),
                (4, _, _) => self.sne_byte(instruction),
                (5, _, _) => self.se_reg(instruction),
                (6, _, _) => self.ld_byte(instruction),
                (7, _, _) => self.add_byte(instruction),
                (8, _, (_, _, 0)) => self.ld_reg(instruction),
                (8, _, (_, _, 1)) => self.or(instruction),
                (8, _, (_, _, 2)) => self.and(instruction),
                (8, _, (_, _, 3)) => self.xor(instruction),
                (8, _, (_, _, 4)) => self.add_reg(instruction),
                (8, _, (_, _, 5)) => self.sub(instruction),
                (8, _, (_, _, 6)) => self.shr(instruction),
                (8, _, (_, _, 7)) => self.subn(instruction),
                (8, _, (_, _, 0xE)) => self.shl(instruction),
                (9, _, _) => self.sne_reg(instruction),
                (0xA, _, _) => self.ld_addr(instruction),
                (0xB, _, _) => self.jp_reg(instruction),
                (0xC, _, _) => self.rnd(instruction),
                (0xD, _, _) => self.drw(instruction),
                (0xE, _, (_, 0x9, 0xE)) => self.skp(instruction),
                (0xE, _, (_, 0xA, 1)) => self.sknp(instruction),
                (0xF, _, (_, 0, 7)) => self.ld_reg_dt(instruction),
                (0xF, _, (_, 0, 0xA)) => self.ld_k(instruction),
                (0xF, _, (_, 1, 5)) => self.ld_dt_reg(instruction),
                (0xF, _, (_, 1, 8)) => self.ld_st(instruction),
                (0xF, _, (_, 1, 0xE)) => self.add_i_reg(instruction),
                (0xF, _, (_, 2, 9)) => self.ld_sprite(instruction),
                (0xF, _, (_, 3, 3)) => self.ld_bcd(instruction),
                (0xF, _, (_, 5, 5)) => self.ld_i(instruction),
                (0xF, _, (_, 6, 3)) => self.ld_regs(instruction),
                (_, bytes, _) => panic!("Unknown instruction - 0x{bytes:04x}"),
            }
        }
    }

    /// 0nnn - SYS addr
    /// Jump to a machine code routine at nnn.
    ///
    /// This instruction is only used on the old computers on which Chip-8 was originally
    /// implemented. It is ignored by modern interpreters.
    fn sys(&mut self, instruction: Instruction) {}

    /// 00E0 - CLS
    /// Clear the display.
    fn cls(&mut self, instruction: Instruction) {}

    /// 00EE - RET
    /// Return from a subroutine.
    ///
    /// The interpreter sets the program counter to the address at the top of the stack,
    /// then subtracts 1 from the stack pointer.
    fn ret(&mut self, instruction: Instruction) {}

    /// 1nnn - JP addr
    /// Jump to location nnn.
    ///
    /// The interpreter sets the program counter to nnn.
    fn jp_addr(&self, instruction: Instruction) {}

    /// 2nnn - CALL addr
    /// Call subroutine at nnn.
    ///
    /// The interpreter increments the stack pointer, then puts the current PC on the
    /// top of the stack. The PC is then set to nnn.
    fn call(&self, instruction: Instruction) {}

    /// 3xkk - SE Vx, byte
    /// Skip next instruction if Vx = kk.
    ///
    /// The interpreter compares register Vx to kk, and if they are equal, increments
    /// the program counter by 2.
    fn se_byte(&self, instruction: Instruction) {}

    /// 4xkk - SNE Vx, byte
    /// Skip next instruction if Vx != kk.
    ///
    /// The interpreter compares register Vx to kk, and if they are not equal, increments
    /// the program counter by 2.
    fn sne_byte(&self, instruction: Instruction) {}

    /// 5xy0 - SE Vx, Vy
    /// Skip next instruction if Vx = Vy.
    ///
    /// The interpreter compares register Vx to register Vy, and if they are equal,
    /// increments the program counter by 2.
    fn se_reg(&self, instruction: Instruction) {}

    /// 6xkk - LD Vx, byte
    /// Set Vx = kk.
    ///
    /// The interpreter puts the value kk into register Vx.
    fn ld_byte(&self, instruction: Instruction) {}

    /// 7xkk - ADD Vx, byte
    /// Set Vx = Vx + kk.
    ///
    /// Adds the value kk to the value of register Vx, then stores the result in Vx.
    fn add_byte(&self, instruction: Instruction) {}

    /// 8xy0 - LD Vx, Vy
    /// Set Vx = Vy.
    ///
    /// Stores the value of register Vy in register Vx.
    fn ld_reg(&self, instruction: Instruction) {}

    /// 8xy1 - OR Vx, Vy
    /// Set Vx = Vx OR Vy.
    ///
    /// Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
    /// A bitwise OR compares the corrseponding bits from two values, and if either bit is 1,
    /// then the same bit in the result is also 1. Otherwise, it is 0.
    fn or(&self, instruction: Instruction) {}

    /// 8xy2 - AND Vx, Vy
    /// Set Vx = Vx AND Vy.
    ///
    /// Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
    /// A bitwise AND compares the corrseponding bits from two values, and if both bits are 1,
    /// then the same bit in the result is also 1. Otherwise, it is 0.
    fn and(&self, instruction: Instruction) {}

    /// 8xy3 - XOR Vx, Vy
    /// Set Vx = Vx XOR Vy.
    ///
    /// Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result
    /// in Vx. An exclusive OR compares the corrseponding bits from two values, and if the
    /// bits are not both the same, then the corresponding bit in the result is set to 1.
    /// Otherwise, it is 0.
    fn xor(&self, instruction: Instruction) {}

    /// 8xy4 - ADD Vx, Vy
    /// Set Vx = Vx + Vy, set VF = carry.
    ///
    /// The values of Vx and Vy are added together. If the result is greater than 8 bits
    /// (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest 8 bits of the result
    /// are kept, and stored in Vx.
    fn add_reg(&self, instruction: Instruction) {}

    /// 8xy5 - SUB Vx, Vy
    /// Set Vx = Vx - Vy, set VF = NOT borrow.
    ///
    /// If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and
    /// the results stored in Vx.
    fn sub(&self, instruction: Instruction) {}

    /// 8xy6 - SHR Vx {, Vy}
    /// Set Vx = Vx SHR 1.
    ///
    /// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then
    /// Vx is divided by 2.
    fn shr(&self, instruction: Instruction) {}

    /// 8xy7 - SUBN Vx, Vy
    /// Set Vx = Vy - Vx, set VF = NOT borrow.
    ///
    /// If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and
    /// the results stored in Vx.
    fn subn(&self, instruction: Instruction) {}

    /// 8xyE - SHL Vx {, Vy}
    /// Set Vx = Vx SHL 1.
    ///
    /// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0.
    /// Then Vx is multiplied by 2.
    fn shl(&self, instruction: Instruction) {}

    /// 9xy0 - SNE Vx, Vy
    /// Skip next instruction if Vx != Vy.
    ///
    /// The values of Vx and Vy are compared, and if they are not equal, the program
    /// counter is increased by 2.
    fn sne_reg(&self, instruction: Instruction) {}

    /// Annn - LD I, addr
    /// Set I = nnn.
    ///
    /// The value of register I is set to nnn.
    fn ld_addr(&self, instruction: Instruction) {}

    /// Bnnn - JP V0, addr
    /// Jump to location nnn + V0.
    ///
    /// The program counter is set to nnn plus the value of V0.
    fn jp_reg(&self, instruction: Instruction) {}

    /// Cxkk - RND Vx, byte
    /// Set Vx = random byte AND kk.
    ///
    /// The interpreter generates a random number from 0 to 255, which is then
    /// ANDed with the value kk. The results are stored in Vx.
    fn rnd(&self, instruction: Instruction) {}

    /// Dxyn - DRW Vx, Vy, nibble
    /// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    ///
    /// The interpreter reads n bytes from memory, starting at the address stored
    /// in I. These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
    /// Sprites are XORed onto the existing screen. If this causes any pixels to
    /// be erased, VF is set to 1, otherwise it is set to 0. If the sprite is positioned
    /// so part of it is outside the coordinates of the display, it wraps around to
    /// the opposite side of the screen.
    fn drw(&self, instruction: Instruction) {}

    /// Ex9E - SKP Vx
    /// Skip next instruction if key with the value of Vx is pressed.
    ///
    /// Checks the keyboard, and if the key corresponding to the value of Vx is
    /// currently in the down position, PC is increased by 2.
    fn skp(&self, instruction: Instruction) {}

    /// ExA1 - SKNP Vx
    /// Skip next instruction if key with the value of Vx is not pressed.
    ///
    /// Checks the keyboard, and if the key corresponding to the value of Vx is
    /// currently in the up position, PC is increased by 2.
    fn sknp(&self, instruction: Instruction) {}

    /// Fx07 - LD Vx, DT
    /// Set Vx = delay timer value.
    ///
    /// The value of DT is placed into Vx.
    fn ld_reg_dt(&self, instruction: Instruction) {}

    /// Fx0A - LD Vx, K
    /// Wait for a key press, store the value of the key in Vx.
    ///
    /// All execution stops until a key is pressed, then the value of that key is
    /// stored in Vx.
    fn ld_k(&self, instruction: Instruction) {}

    /// Fx15 - LD DT, Vx
    /// Set delay timer = Vx.
    ///
    /// DT is set equal to the value of Vx.
    fn ld_dt_reg(&self, instruction: Instruction) {}

    /// Fx18 - LD ST, Vx
    /// Set sound timer = Vx.
    ///
    /// ST is set equal to the value of Vx.
    fn ld_st(&self, instruction: Instruction) {}

    /// Fx1E - ADD I, Vx
    /// Set I = I + Vx.
    ///
    /// The values of I and Vx are added, and the results are stored in I.
    fn add_i_reg(&self, instruction: Instruction) {}

    /// Fx29 - LD F, Vx
    /// Set I = location of sprite for digit Vx.
    ///
    /// The value of I is set to the location for the hexadecimal sprite corresponding
    /// to the value of Vx. See section 2.4, Display, for more information on the
    /// Chip-8 hexadecimal font.
    fn ld_sprite(&self, instruction: Instruction) {}

    /// Fx33 - LD B, Vx
    /// Store BCD representation of Vx in memory locations I, I+1, and I+2.
    ///
    /// The interpreter takes the decimal value of Vx, and places the hundreds digit
    /// in memory at location in I, the tens digit at location I+1, and the ones
    /// digit at location I+2.
    fn ld_bcd(&self, instruction: Instruction) {}

    /// Fx55 - LD [I], Vx
    /// Store registers V0 through Vx in memory starting at location I.
    ///
    /// The interpreter copies the values of registers V0 through Vx into memory,
    /// starting at the address in I.
    fn ld_i(&self, instruction: Instruction) {}

    /// Fx65 - LD Vx, [I]
    /// Read registers V0 through Vx from memory starting at location I.
    ///
    /// The interpreter reads values from memory starting at location I into
    /// registers V0 through Vx.
    fn ld_regs(&self, instruction: Instruction) {}

    fn next_instruction(&mut self) -> Instruction {
        let instruction_bytes = u16::from_be_bytes([
            self.memory.read(self.program_counter),
            self.memory.read(self.program_counter + 1),
        ]);
        self.program_counter += 2;
        Instruction::new(instruction_bytes)
    }
}
