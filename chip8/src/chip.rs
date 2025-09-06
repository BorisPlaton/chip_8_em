use crate::display::Display;
use crate::instruction::Instruction;
use crate::keyboard::Keyboard;
use crate::memory::{FontSize, Memory};
use crate::modes::ChipMode;
use crate::registers::memory::MemoryRegister;
use crate::registers::timer::TimerRegister;
use crate::rom::Rom;
use crate::stack::Stack;
use std::collections::HashMap;

pub struct Chip8<'a> {
    memory: Memory<'a>,
    stack: Stack,
    display: Display,
    keyboard: Keyboard,
    /// General purpose registers.
    registers: HashMap<u8, u8>,
    /// `I` register is generally used to store memory addresses, so only
    /// the lowest (rightmost) 12 bits are usually used.
    i_register: MemoryRegister,
    /// Delay timer register.
    dt_register: TimerRegister,
    /// Sound timer register.
    st_register: TimerRegister,
    /// PC is used to store the currently executing address.
    program_counter: u16,
    mode: &'a ChipMode,
}

impl<'a> Chip8<'a> {
    const TICKS_PER_FRAME: u16 = 950;

    pub fn new(rom: Rom, mode: &'a ChipMode) -> Chip8<'a> {
        Chip8 {
            memory: Memory::new(rom.content(), mode),
            stack: Stack::default(),
            display: Display::default(),
            keyboard: Keyboard::default(),
            i_register: MemoryRegister::default(),
            dt_register: TimerRegister::default(),
            st_register: TimerRegister::default(),
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
            mode,
        }
    }

    pub fn run<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut Keyboard, &Display, u8),
    {
        loop {
            for _ in 0..Self::TICKS_PER_FRAME {
                self.execute();
            }
            self.dt_register.tick();
            self.st_register.tick();
            callback(&mut self.keyboard, &self.display, self.st_register.get());
        }
    }

    fn execute(&mut self) {
        let instruction = self.next_instruction();
        match (&self.mode, instruction.nibbles()) {
            (ChipMode::SuperChip, (0, 0, 0xC, n)) if n > 0 => self.scroll_n_lines_down(instruction),
            (_, (0, 0, 0xE, 0)) => self.cls(),
            (_, (0, 0, 0xE, 0xE)) => self.ret(),
            (ChipMode::SuperChip, (0, 0, 0xF, 0xB)) => self.scroll_display_4_px_right(),
            (ChipMode::SuperChip, (0, 0, 0xF, 0xC)) => self.scroll_display_4_px_left(),
            (ChipMode::SuperChip, (0, 0, 0xF, 0xD)) => self.exit_interpreter(),
            (ChipMode::SuperChip, (0, 0, 0xF, 0xE)) => self.disable_display_extended_mode(),
            (ChipMode::SuperChip, (0, 0, 0xF, 0xF)) => self.enable_display_extended_mode(),
            (ChipMode::Chip8, (0, _, _, _)) => self.jp_addr(instruction),
            (_, (1, ..)) => self.jp_addr(instruction),
            (_, (2, ..)) => self.call_addr(instruction),
            (_, (3, ..)) => self.se_vx_byte(instruction),
            (_, (4, ..)) => self.sne_vx_byte(instruction),
            (_, (5, ..)) => self.se_vx_vy(instruction),
            (_, (6, ..)) => self.ld_vx_byte(instruction),
            (_, (7, ..)) => self.add_vx_byte(instruction),
            (_, (8, .., 0)) => self.ld_vx_vy(instruction),
            (_, (8, .., 1)) => self.or_vx_vy(instruction),
            (_, (8, .., 2)) => self.and_vx_vy(instruction),
            (_, (8, .., 3)) => self.xor_vx_vy(instruction),
            (_, (8, .., 4)) => self.add_vx_vy(instruction),
            (_, (8, .., 5)) => self.sub_vx_vy(instruction),
            (_, (8, .., 6)) => self.shr_vx(instruction),
            (_, (8, .., 7)) => self.subn_vx_vy(instruction),
            (_, (8, .., 0xE)) => self.shl_vx(instruction),
            (_, (9, ..)) => self.sne_vx_vy(instruction),
            (_, (0xA, ..)) => self.ld_i_addr(instruction),
            (_, (0xB, ..)) => self.jp_vo_addr(instruction),
            (_, (0xC, ..)) => self.rnd_vx_byte(instruction),
            (_, (0xD, ..)) => self.drw_vx_vy_n(instruction),
            (_, (0xE, _, 0x9, 0xE)) => self.skp_vx(instruction),
            (_, (0xE, _, 0xA, 1)) => self.sknp_vx(instruction),
            (_, (0xF, _, 0, 7)) => self.ld_vx_dt(instruction),
            (_, (0xF, _, 0, 0xA)) => self.ld_vx_k(instruction),
            (_, (0xF, _, 1, 5)) => self.ld_dt_vx(instruction),
            (_, (0xF, _, 1, 8)) => self.ld_st_vx(instruction),
            (_, (0xF, _, 1, 0xE)) => self.add_i_vx(instruction),
            (_, (0xF, _, 2, 9)) => self.ld_f_vx(instruction),
            (ChipMode::SuperChip, (0xF, _, 3, 0)) => self.load_10_byte_font_to_i(instruction),
            (_, (0xF, _, 3, 3)) => self.ld_b_vx(instruction),
            (_, (0xF, _, 5, 5)) => self.ld_i_vx(instruction),
            (_, (0xF, _, 6, 5)) => self.ld_vx_i(instruction),
            (ChipMode::SuperChip, (0xF, x, 7, 5)) if x <= 7 => self.load_rpl_flags(instruction),
            (ChipMode::SuperChip, (0xF, x, 8, 5)) if x <= 7 => self.read_rpl_flags(instruction),
            (_, bytes) => {
                let lo_byte = bytes.3 + (bytes.2 << 4);
                let hi_byte = bytes.1 + (bytes.0 << 4);
                panic!(
                    "Unknown instruction 0x{:04X} for {}",
                    u16::from_be_bytes([hi_byte, lo_byte]),
                    self.mode,
                )
            }
        }
    }

    /// 00CN - Scroll display N lines down
    fn scroll_n_lines_down(&mut self, instruction: Instruction) {
        self.display.scroll_n_lines_down(instruction.n());
    }

    /// 00E0 - CLS
    /// Clear the display.
    fn cls(&mut self) {
        self.display.clear();
    }

    /// 00EE - RET
    /// Return from a subroutine.
    ///
    /// The interpreter sets the program counter to the address at the top of the stack,
    /// then subtracts 1 from the stack pointer.
    fn ret(&mut self) {
        self.program_counter = self.stack.pull();
    }

    /// 00FB - Scroll display 4 pixels right
    fn scroll_display_4_px_right(&mut self) {
        self.display.scroll_4_px_right();
    }

    /// 00FC - Scroll display 4 pixels left
    fn scroll_display_4_px_left(&mut self) {
        self.display.scroll_4_px_left();
    }

    /// 00FD - Exit interpreter
    fn exit_interpreter(&self) {
        std::process::exit(0);
    }

    /// 00FE - Disable extended screen mode for full-screen graphics.
    fn disable_display_extended_mode(&mut self) {
        self.display.disable_extended_mode();
    }

    /// 00FF - Enable extended screen mode for full-screen graphics.
    fn enable_display_extended_mode(&mut self) {
        self.display.enable_extended_mode();
    }

    /// 1nnn - JP addr
    /// Jump to location nnn.
    ///
    /// The interpreter sets the program counter to nnn.
    fn jp_addr(&mut self, instruction: Instruction) {
        self.program_counter = instruction.nnn();
    }

    /// 2nnn - CALL addr
    /// Call subroutine at nnn.
    ///
    /// The interpreter increments the stack pointer, then puts the current PC on the
    /// top of the stack. The PC is then set to nnn.
    fn call_addr(&mut self, instruction: Instruction) {
        self.stack.push(self.program_counter);
        self.program_counter = instruction.nnn();
    }

    /// 3xkk - SE Vx, byte
    /// Skip next instruction if Vx = kk.
    ///
    /// The interpreter compares register Vx to kk, and if they are equal, increments
    /// the program counter by 2.
    fn se_vx_byte(&mut self, instruction: Instruction) {
        let register_x = self.registers[&instruction.x()];
        if register_x == instruction.kk() {
            self.program_counter += 2;
        }
    }

    /// 4xkk - SNE Vx, byte
    /// Skip next instruction if Vx != kk.
    ///
    /// The interpreter compares register Vx to kk, and if they are not equal, increments
    /// the program counter by 2.
    fn sne_vx_byte(&mut self, instruction: Instruction) {
        let register_x = self.registers[&instruction.x()];
        if register_x != instruction.kk() {
            self.program_counter += 2;
        }
    }

    /// 5xy0 - SE Vx, Vy
    /// Skip next instruction if Vx = Vy.
    ///
    /// The interpreter compares register Vx to register Vy, and if they are equal,
    /// increments the program counter by 2.
    fn se_vx_vy(&mut self, instruction: Instruction) {
        let register_x = self.registers[&instruction.x()];
        let register_y = self.registers[&instruction.y()];
        if register_x == register_y {
            self.program_counter += 2;
        }
    }

    /// 6xkk - LD Vx, byte
    /// Set Vx = kk.
    ///
    /// The interpreter puts the value kk into register Vx.
    fn ld_vx_byte(&mut self, instruction: Instruction) {
        self.registers.insert(instruction.x(), instruction.kk());
    }

    /// 7xkk - ADD Vx, byte
    /// Set Vx = Vx + kk.
    ///
    /// Adds the value kk to the value of register Vx, then stores the result in Vx.
    fn add_vx_byte(&mut self, instruction: Instruction) {
        let register_x = self.registers[&instruction.x()];
        self.registers
            .insert(instruction.x(), register_x.wrapping_add(instruction.kk()));
    }

    /// 8xy0 - LD Vx, Vy
    /// Set Vx = Vy.
    ///
    /// Stores the value of register Vy in register Vx.
    fn ld_vx_vy(&mut self, instruction: Instruction) {
        self.registers
            .insert(instruction.x(), self.registers[&instruction.y()]);
    }

    /// 8xy1 - OR Vx, Vy
    /// Set Vx = Vx OR Vy.
    ///
    /// Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
    /// A bitwise OR compares the corresponding bits from two values, and if either bit is 1,
    /// then the same bit in the result is also 1. Otherwise, it is 0.
    fn or_vx_vy(&mut self, instruction: Instruction) {
        let register_x = self.registers[&instruction.x()];
        let register_y = self.registers[&instruction.y()];
        self.registers
            .insert(instruction.x(), register_x | register_y);
        if self.mode == &ChipMode::Chip8 {
            self.registers.insert(0xF, 0);
        }
    }

    /// 8xy2 - AND Vx, Vy
    /// Set Vx = Vx AND Vy.
    ///
    /// Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
    /// A bitwise AND compares the corresponding bits from two values, and if both bits are 1,
    /// then the same bit in the result is also 1. Otherwise, it is 0.
    fn and_vx_vy(&mut self, instruction: Instruction) {
        let register_x = self.registers[&instruction.x()];
        let register_y = self.registers[&instruction.y()];
        self.registers
            .insert(instruction.x(), register_x & register_y);
        if self.mode == &ChipMode::Chip8 {
            self.registers.insert(0xF, 0);
        }
    }

    /// 8xy3 - XOR Vx, Vy
    /// Set Vx = Vx XOR Vy.
    ///
    /// Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result
    /// in Vx. An exclusive OR compares the corresponding bits from two values, and if the
    /// bits are not both the same, then the corresponding bit in the result is set to 1.
    /// Otherwise, it is 0.
    fn xor_vx_vy(&mut self, instruction: Instruction) {
        let register_x = self.registers[&instruction.x()];
        let register_y = self.registers[&instruction.y()];
        self.registers
            .insert(instruction.x(), register_x ^ register_y);
        if self.mode == &ChipMode::Chip8 {
            self.registers.insert(0xF, 0);
        }
    }

    /// 8xy4 - ADD Vx, Vy
    /// Set Vx = Vx + Vy, set VF = carry.
    ///
    /// The values of Vx and Vy are added together. If the result is greater than 8 bits
    /// (i.e., > 255) VF is set to 1, otherwise 0. Only the lowest 8 bits of the result
    /// are kept, and stored in Vx.
    fn add_vx_vy(&mut self, instruction: Instruction) {
        let register_x = self.registers[&instruction.x()];
        let register_y = self.registers[&instruction.y()];
        let (result, carry_flag) = register_x.overflowing_add(register_y);
        self.registers.insert(instruction.x(), result);
        self.registers.insert(0xF, carry_flag as u8);
    }

    /// 8xy5 - SUB Vx, Vy
    /// Set Vx = Vx - Vy, set VF = NOT borrow.
    ///
    /// If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and
    /// the results stored in Vx.
    fn sub_vx_vy(&mut self, instruction: Instruction) {
        let register_x = self.registers[&instruction.x()];
        let register_y = self.registers[&instruction.y()];
        self.registers
            .insert(instruction.x(), register_x.wrapping_sub(register_y));
        self.registers.insert(0xF, (register_x >= register_y) as u8);
    }

    /// 8xy6 - SHR Vx {, Vy}
    /// Set Vx = Vx SHR 1.
    ///
    /// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then
    /// Vx is divided by 2.
    fn shr_vx(&mut self, instruction: Instruction) {
        let register_value = self.registers[&match self.mode {
            ChipMode::Chip8 => instruction.y(),
            ChipMode::SuperChip => instruction.x(),
        }];
        self.registers.insert(instruction.x(), register_value >> 1);
        self.registers.insert(0xF, register_value & 1);
    }

    /// 8xy7 - SUBN Vx, Vy
    /// Set Vx = Vy - Vx, set VF = NOT borrow.
    ///
    /// If Vy >= Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and
    /// the results stored in Vx.
    fn subn_vx_vy(&mut self, instruction: Instruction) {
        let register_x = self.registers[&instruction.x()];
        let register_y = self.registers[&instruction.y()];
        self.registers
            .insert(instruction.x(), register_y.wrapping_sub(register_x));
        self.registers.insert(0xF, (register_y >= register_x) as u8);
    }

    /// 8xyE - SHL Vx {, Vy}
    /// Set Vx = Vx SHL 1.
    ///
    /// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0.
    /// Then Vx is multiplied by 2.
    fn shl_vx(&mut self, instruction: Instruction) {
        let register_value = self.registers[&match self.mode {
            ChipMode::Chip8 => instruction.y(),
            ChipMode::SuperChip => instruction.x(),
        }];
        self.registers.insert(instruction.x(), register_value << 1);
        self.registers.insert(
            0xF,
            if register_value & 0b1000_0000 != 0 {
                1
            } else {
                0
            },
        );
    }

    /// 9xy0 - SNE Vx, Vy
    /// Skip next instruction if Vx != Vy.
    ///
    /// The values of Vx and Vy are compared, and if they are not equal, the program
    /// counter is increased by 2.
    fn sne_vx_vy(&mut self, instruction: Instruction) {
        let register_x = self.registers[&instruction.x()];
        let register_y = self.registers[&instruction.y()];
        if register_x != register_y {
            self.program_counter += 2;
        }
    }

    /// Annn - LD I, addr
    /// Set I = nnn.
    ///
    /// The value of register I is set to nnn.
    fn ld_i_addr(&mut self, instruction: Instruction) {
        self.i_register.set(instruction.nnn());
    }

    /// *CHIP-8*
    /// Bnnn - JP V0, addr
    /// Jump to address nnn + V0.
    ///
    /// The program counter is set to nnn plus the value of V0.
    ///
    /// *SCHIP*
    /// Bxnn - JP Vx, addr
    /// Jump to address XNN + vX
    ///
    /// The program counter is set to xnn plus the value of Vx.
    fn jp_vo_addr(&mut self, instruction: Instruction) {
        let register_val = self.registers[&match &self.mode {
            ChipMode::Chip8 => 0,
            ChipMode::SuperChip => instruction.x(),
        }];
        self.program_counter = instruction.nnn() + register_val as u16;
    }

    /// Cxkk - RND Vx, byte
    /// Set Vx = random byte AND kk.
    ///
    /// The interpreter generates a random number from 0 to 255, which is then
    /// ANDed with the value kk. The results are stored in Vx.
    fn rnd_vx_byte(&mut self, instruction: Instruction) {
        self.registers
            .insert(instruction.x(), rand::random::<u8>() & instruction.kk());
    }

    /// *CHIP-8*
    /// Dxyn - DRW Vx, Vy, nibble
    /// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    ///
    /// *SCHIP*
    /// If N=0 and extended mode, show 16x16 sprite.
    ///
    /// The interpreter reads n bytes from memory, starting at the address stored
    /// in I. These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
    /// Sprites are XORed onto the existing screen. If this causes any pixels to
    /// be erased, VF is set to 1, otherwise it is set to 0. If the sprite is positioned
    /// so part of it is outside the coordinates of the display, it wraps around to
    /// the opposite side of the screen.
    fn drw_vx_vy_n(&mut self, instruction: Instruction) {
        let pixel_erased = if instruction.n() == 0 && self.display.is_extended_mode() {
            let sprite_bytes = (0..32u16)
                .into_iter()
                .map(|i| self.memory.read(self.i_register.add(i)))
                .collect::<Vec<u8>>()
                .chunks_exact(2)
                .map(|sprite_bytes| u16::from_be_bytes(sprite_bytes.try_into().unwrap()))
                .collect::<Vec<u16>>()
                .try_into()
                .unwrap();
            self.display.draw_16_16_sprite(
                self.registers[&instruction.x()] as usize,
                self.registers[&instruction.y()] as usize,
                sprite_bytes,
            )
        } else {
            let sprite_bytes: Vec<_> = (0..instruction.n() as u16)
                .into_iter()
                .map(|i| self.memory.read(self.i_register.add(i)))
                .collect();
            self.display.draw_sprite(
                self.registers[&instruction.x()] as usize,
                self.registers[&instruction.y()] as usize,
                &sprite_bytes,
            )
        };
        self.registers.insert(0xF, pixel_erased as u8);
    }

    /// Ex9E - SKP Vx
    /// Skip next instruction if key with the value of Vx is pressed.
    ///
    /// Checks the keyboard, and if the key corresponding to the value of Vx is
    /// currently in the down position, PC is increased by 2.
    fn skp_vx(&mut self, instruction: Instruction) {
        let register_x = self.registers[&instruction.x()];
        if self.keyboard.is_key_pressed(register_x) {
            self.program_counter += 2
        };
    }

    /// ExA1 - SKNP Vx
    /// Skip next instruction if key with the value of Vx is not pressed.
    ///
    /// Checks the keyboard, and if the key corresponding to the value of Vx is
    /// currently in the up position, PC is increased by 2.
    fn sknp_vx(&mut self, instruction: Instruction) {
        let register_x = self.registers[&instruction.x()];
        if !self.keyboard.is_key_pressed(register_x) {
            self.program_counter += 2
        };
    }

    /// Fx07 - LD Vx, DT
    /// Set Vx = delay timer value.
    ///
    /// The value of DT is placed into Vx.
    fn ld_vx_dt(&mut self, instruction: Instruction) {
        self.registers
            .insert(instruction.x(), self.dt_register.get());
    }

    /// Fx0A - LD Vx, K
    /// Wait for a key press, store the value of the key in Vx.
    ///
    /// All execution stops until a key is pressed, then the value of that key is
    /// stored in Vx.
    fn ld_vx_k(&mut self, instruction: Instruction) {
        if let Some(pressed_key) = self.keyboard.pressed_key() {
            self.registers.insert(instruction.x(), pressed_key);
        } else {
            self.program_counter -= 2;
        };
    }

    /// Fx15 - LD DT, Vx
    /// Set delay timer = Vx.
    ///
    /// DT is set equal to the value of Vx.
    fn ld_dt_vx(&mut self, instruction: Instruction) {
        let register_x = self.registers[&instruction.x()];
        self.dt_register.set(register_x);
    }

    /// Fx18 - LD ST, Vx
    /// Set sound timer = Vx.
    ///
    /// ST is set equal to the value of Vx.
    fn ld_st_vx(&mut self, instruction: Instruction) {
        let register_x = self.registers[&instruction.x()];
        self.st_register.set(register_x);
    }

    /// Fx1E - ADD I, Vx
    /// Set I = I + Vx.
    ///
    /// The values of I and Vx are added, and the results are stored in `I`.
    fn add_i_vx(&mut self, instruction: Instruction) {
        let register_x = self.registers[&instruction.x()];
        self.i_register.set(self.i_register.add(register_x as u16));
    }

    /// Fx29 - LD F, Vx
    /// Set I = location of sprite for digit Vx.
    ///
    /// The value of I is set to the location for the hexadecimal sprite corresponding
    /// to the value of Vx. See section 2.4, Display, for more information on the
    /// Chip-8 hexadecimal font.
    fn ld_f_vx(&mut self, instruction: Instruction) {
        let register_x = self.registers[&instruction.x()];
        self.i_register
            .set(self.memory.get_font_address(register_x, FontSize::Standard));
    }

    /// Fx30 - Point I to 10-byte font sprite for digit VX (0..F)
    fn load_10_byte_font_to_i(&mut self, instruction: Instruction) {
        let register_x = self.registers[&instruction.x()];
        self.i_register
            .set(self.memory.get_font_address(register_x, FontSize::Extended));
    }

    /// Fx33 - LD B, Vx
    /// Store BCD representation of Vx in memory locations I, I+1, and I+2.
    ///
    /// The interpreter takes the decimal value of Vx, and places the hundreds digit
    /// in memory at location in I, the tens digit at location I+1, and the ones
    /// digit at location I+2.
    fn ld_b_vx(&mut self, instruction: Instruction) {
        let register_x = self.registers[&instruction.x()];
        self.memory.write(self.i_register.get(), register_x / 100);
        self.memory
            .write(self.i_register.add(1), (register_x / 10) % 10);
        self.memory.write(self.i_register.add(2), register_x % 10);
    }

    /// Fx55 - LD [I], Vx
    /// Store registers V0 through Vx in memory starting at location I.
    ///
    /// The interpreter copies the values of registers V0 through Vx into memory,
    /// starting at the address in `I`.
    fn ld_i_vx(&mut self, instruction: Instruction) {
        (0..=instruction.x()).for_each(|register| {
            self.memory.write(
                self.i_register.add(register as u16),
                *self.registers.get(&register).unwrap(),
            );
        });
        if self.mode == &ChipMode::Chip8 {
            self.i_register
                .set(self.i_register.get() + instruction.x() as u16 + 1);
        }
    }

    /// Fx65 - LD Vx, [I]
    /// Read registers V0 through Vx from memory starting at location I.
    ///
    /// The interpreter reads values from memory starting at location I into
    /// registers V0 through Vx.
    fn ld_vx_i(&mut self, instruction: Instruction) {
        (0..=instruction.x()).for_each(|register| {
            self.registers.insert(
                register,
                self.memory.read(self.i_register.add(register as u16)),
            );
        });
        if self.mode == &ChipMode::Chip8 {
            self.i_register
                .set(self.i_register.get() + instruction.x() as u16 + 1);
        }
    }

    /// FX75 - Store V0..VX in RPL user flags (X <= 7)
    fn load_rpl_flags(&mut self, instruction: Instruction) {
        self.memory.write_rpl_flags(
            &self
                .registers
                .iter()
                .filter(|(i, _)| i < &&instruction.x())
                .map(|(i, _)| self.registers[i])
                .collect::<Vec<_>>(),
        );
    }

    /// FX85 - Read V0..VX from RPL user flags (X <= 7)
    fn read_rpl_flags(&mut self, instruction: Instruction) {
        self.memory
            .read_rpl_flags()
            .iter()
            .filter(|x| x < &&instruction.x())
            .enumerate()
            .for_each(|(i, &x)| {
                self.registers.insert(i as u8, x);
            });
    }

    fn next_instruction(&mut self) -> Instruction {
        let instruction_bytes = u16::from_be_bytes([
            self.memory.read(self.program_counter),
            self.memory.read(self.program_counter + 1),
        ]);
        self.program_counter += 2;
        Instruction::new(instruction_bytes)
    }
}
