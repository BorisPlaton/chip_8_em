use crate::display::{Display, Plane, ScreenResolution};
use crate::instruction::Instruction;
use crate::keyboard::Keyboard;
use crate::memory::Memory;
use crate::platform::{ChipMode, Quirks};
use crate::registers::memory::MemoryRegister;
use crate::registers::timer::TimerRegister;
use crate::rom::Rom;
use crate::stack::Stack;
use std::collections::{HashMap, HashSet};
use std::time::Duration;

pub struct Chip8<'a> {
    memory: Memory<'a>,
    stack: Stack,
    display: Display<'a>,
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

    audio_buffer: [u8; 16],
    pitch: u16,

    ticks_per_frame: u32,
    mode: &'a ChipMode,
    quirks: &'a HashSet<Quirks>,
    sleep_time: Option<u8>,
}

impl<'a> Chip8<'a> {
    pub fn new(
        rom: Rom,
        mode: &'a ChipMode,
        quirks: &'a HashSet<Quirks>,
        ticks_per_frame: u32,
        sleep_time: Option<u8>,
    ) -> Chip8<'a> {
        let memory = Memory::new(rom.content(), mode);
        let memory_size = memory.get_memory_size();
        Chip8 {
            memory,
            stack: Stack::new(memory_size),
            display: Display::new(quirks),
            keyboard: Keyboard::default(),
            i_register: MemoryRegister::new(memory_size),
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
            audio_buffer: [0xFF; 16],
            pitch: 8000,
            mode,
            quirks,
            ticks_per_frame,
            sleep_time,
        }
    }

    pub fn run<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut Keyboard, &Display, u8, &[u8], u16),
    {
        loop {
            (0..self.ticks_per_frame).for_each(|_| {
                self.execute();
                if let Some(sleep_time) = self.sleep_time {
                    std::thread::sleep(Duration::from_micros(sleep_time as u64));
                }
            });

            self.dt_register.tick();
            self.st_register.tick();

            callback(
                &mut self.keyboard,
                &self.display,
                self.st_register.get(),
                &self.audio_buffer,
                self.pitch,
            );
        }
    }

    fn execute(&mut self) {
        let instruction = self.next_instruction();
        match (&self.mode, instruction.nibbles()) {
            (ChipMode::SuperChip | ChipMode::XOChip, (0, 0, 0xC, n)) if n > 0 => {
                self.scroll_n_lines_down(instruction)
            }
            (ChipMode::XOChip, (0, 0, 0xD, _)) => self.scroll_n_lines_up(instruction),
            (_, (0, 0, 0xE, 0)) => self.cls(),
            (_, (0, 0, 0xE, 0xE)) => self.ret(),
            (ChipMode::SuperChip | ChipMode::XOChip, (0, 0, 0xF, 0xB)) => {
                self.scroll_display_4_px_right()
            }
            (ChipMode::SuperChip | ChipMode::XOChip, (0, 0, 0xF, 0xC)) => {
                self.scroll_display_4_px_left()
            }
            (ChipMode::SuperChip | ChipMode::XOChip, (0, 0, 0xF, 0xD)) => self.exit_interpreter(),
            (ChipMode::SuperChip | ChipMode::XOChip, (0, 0, 0xF, 0xE)) => self.disable_hires(),
            (ChipMode::SuperChip | ChipMode::XOChip, (0, 0, 0xF, 0xF)) => self.enable_hires(),
            (ChipMode::Chip8, (0, _, _, _)) => self.jp_addr(instruction),
            (_, (1, ..)) => self.jp_addr(instruction),
            (_, (2, ..)) => self.call_addr(instruction),
            (_, (3, ..)) => self.se_vx_byte(instruction),
            (_, (4, ..)) => self.sne_vx_byte(instruction),
            (ChipMode::XOChip, (5, .., 2)) => self.save_registers_range(instruction),
            (ChipMode::XOChip, (5, .., 3)) => self.load_registers_range(instruction),
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
            (_, (9, .., 0)) => self.sne_vx_vy(instruction),
            (_, (0xA, ..)) => self.ld_i_addr(instruction),
            (_, (0xB, ..)) => self.jp_vo_addr(instruction),
            (_, (0xC, ..)) => self.rnd_vx_byte(instruction),
            (_, (0xD, ..)) => self.drw_vx_vy_n(instruction),
            (_, (0xE, _, 0x9, 0xE)) => self.skp_vx(instruction),
            (_, (0xE, _, 0xA, 1)) => self.sknp_vx(instruction),
            (ChipMode::XOChip, (0xF, 0, 0, 0)) => self.load_i(),
            (ChipMode::XOChip, (0xF, _, 0, 1)) => self.set_plane(instruction),
            (ChipMode::XOChip, (0xF, 0, 0, 2)) => self.load_audio_buffer(),
            (_, (0xF, _, 0, 7)) => self.ld_vx_dt(instruction),
            (_, (0xF, _, 0, 0xA)) => self.ld_vx_k(instruction),
            (_, (0xF, _, 1, 5)) => self.ld_dt_vx(instruction),
            (_, (0xF, _, 1, 8)) => self.ld_st_vx(instruction),
            (_, (0xF, _, 1, 0xE)) => self.add_i_vx(instruction),
            (_, (0xF, _, 2, 9)) => self.ld_f_vx(instruction),
            (ChipMode::SuperChip | ChipMode::XOChip, (0xF, _, 3, 0)) => {
                self.load_10_byte_font_to_i(instruction)
            }
            (_, (0xF, _, 3, 3)) => self.ld_b_vx(instruction),
            (ChipMode::XOChip, (0xF, _, 3, 0xA)) => self.set_pitch(instruction),
            (_, (0xF, _, 5, 5)) => self.ld_i_vx(instruction),
            (_, (0xF, _, 6, 5)) => self.ld_vx_i(instruction),
            (ChipMode::SuperChip | ChipMode::XOChip, (0xF, _, 7, 5)) => {
                self.load_rpl_flags(instruction)
            }
            (ChipMode::SuperChip | ChipMode::XOChip, (0xF, _, 8, 5)) => {
                self.read_rpl_flags(instruction)
            }
            _ => {
                panic!(
                    "Unknown instruction 0x{:04X} for {}",
                    instruction.value(),
                    self.mode,
                )
            }
        }
    }

    /// 00CN - Scroll display N lines down
    fn scroll_n_lines_down(&mut self, instruction: Instruction) {
        self.display.scroll_n_lines_down(instruction.n());
    }

    /// 0x00DN - scroll the contents of the display up by N pixels.
    fn scroll_n_lines_up(&mut self, instruction: Instruction) {
        self.display.scroll_n_lines_up(instruction.n());
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

    /// 00FE - Disable high resolution screen mode for full-screen graphics.
    fn disable_hires(&mut self) {
        self.display.disable_hires();
    }

    /// 00FF - Enable high resolution screen mode for full-screen graphics.
    fn enable_hires(&mut self) {
        self.display.enable_hires();
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
            self.skip_next_instruction();
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
            self.skip_next_instruction();
        }
    }

    /// 0x5XY2 - Save an inclusive range of registers vx - vy to memory starting at `I`.
    fn save_registers_range(&mut self, instruction: Instruction) {
        let range = if instruction.x() > instruction.y() {
            Box::new((instruction.y()..=instruction.x()).rev()) as Box<dyn Iterator<Item = _>>
        } else {
            Box::new((instruction.x()..=instruction.y()).into_iter()) as Box<dyn Iterator<Item = _>>
        };
        range.enumerate().for_each(|(i, register)| {
            self.memory
                .write(self.i_register.add(i as u16), self.registers[&register]);
        });
    }

    /// 0x5XY3 - Load an inclusive range of registers vx - vy from memory starting at `I`.
    fn load_registers_range(&mut self, instruction: Instruction) {
        let range = if instruction.x() > instruction.y() {
            Box::new((instruction.y()..=instruction.x()).rev()) as Box<dyn Iterator<Item = _>>
        } else {
            Box::new((instruction.x()..=instruction.y()).into_iter()) as Box<dyn Iterator<Item = _>>
        };
        range.enumerate().for_each(|(i, register)| {
            self.registers
                .insert(register, self.memory.read(self.i_register.add(i as u16)));
        });
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
            self.skip_next_instruction();
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
        if self.quirks.contains(&Quirks::BinaryOpResetVF) {
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
        if self.quirks.contains(&Quirks::BinaryOpResetVF) {
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
        if self.quirks.contains(&Quirks::BinaryOpResetVF) {
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
    /// If Vx >= Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and
    /// the results stored in Vx.
    fn sub_vx_vy(&mut self, instruction: Instruction) {
        let register_x = self.registers[&instruction.x()];
        let register_y = self.registers[&instruction.y()];
        let (result, carry_flag) = register_x.overflowing_sub(register_y);
        self.registers.insert(instruction.x(), result);
        self.registers.insert(0xF, !carry_flag as u8);
    }

    /// 8xy6 - SHR Vx {, Vy}
    /// Set Vx = Vx SHR 1.
    ///
    /// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then
    /// Vx is divided by 2.
    fn shr_vx(&mut self, instruction: Instruction) {
        let target_register = if self.quirks.contains(&Quirks::ShiftIgnoreVY) {
            instruction.x()
        } else {
            instruction.y()
        };
        let register_value = self.registers[&target_register];
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
        let (result, carry_flag) = register_y.overflowing_sub(register_x);
        self.registers.insert(instruction.x(), result);
        self.registers.insert(0xF, !carry_flag as u8);
    }

    /// 8xyE - SHL Vx {, Vy}
    /// Set Vx = Vx SHL 1.
    ///
    /// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0.
    /// Then Vx is multiplied by 2.
    fn shl_vx(&mut self, instruction: Instruction) {
        let target_register = if self.quirks.contains(&Quirks::ShiftIgnoreVY) {
            instruction.x()
        } else {
            instruction.y()
        };
        let register_value = self.registers[&target_register];
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
            self.skip_next_instruction();
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
        let target_register = if self.quirks.contains(&Quirks::JumpWithX) {
            instruction.x()
        } else {
            0
        };
        let register_value = self.registers[&target_register];
        self.program_counter = instruction.nnn() + register_value as u16;
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
    /// If N=0 and hires mode, show 16x16 sprite.
    ///
    /// The interpreter reads n bytes from memory, starting at the address stored
    /// in I. These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
    /// Sprites are XORed onto the existing screen. If this causes any pixels to
    /// be erased, VF is set to 1, otherwise it is set to 0. If the sprite is positioned
    /// so part of it is outside the coordinates of the display, it wraps around to
    /// the opposite side of the screen.
    fn drw_vx_vy_n(&mut self, instruction: Instruction) {
        let pixel_erased = match (self.mode, instruction.n()) {
            (_, n) if n != 0 => {
                let sprites_to_draw = match self.display.get_current_plane() {
                    Plane::First | Plane::Second => vec![(
                        *self.display.get_current_plane(),
                        self.memory.read_n_bytes(self.i_register.get(), n as u16),
                    )],
                    Plane::Both => vec![
                        (
                            Plane::First,
                            self.memory.read_n_bytes(self.i_register.get(), n as u16),
                        ),
                        (
                            Plane::Second,
                            self.memory
                                .read_n_bytes(self.i_register.add(n as u16), n as u16),
                        ),
                    ],
                };
                sprites_to_draw
                    .into_iter()
                    .map(|(plane, sprite)| {
                        self.display.draw_sprite(
                            self.registers[&instruction.x()] as usize,
                            self.registers[&instruction.y()] as usize,
                            &sprite,
                            plane,
                        )
                    })
                    .fold(false, |acc, is_pixel_erased| acc || is_pixel_erased)
            }
            (ChipMode::SuperChip | ChipMode::XOChip, 0) => {
                let sprites_to_draw = match self.display.get_current_plane() {
                    Plane::First | Plane::Second => vec![(
                        *self.display.get_current_plane(),
                        self.memory.read_n_2bytes(self.i_register.get(), 16),
                    )],
                    Plane::Both => vec![
                        (
                            Plane::First,
                            self.memory.read_n_2bytes(self.i_register.get(), 16),
                        ),
                        (
                            Plane::Second,
                            self.memory.read_n_2bytes(self.i_register.add(32), 16),
                        ),
                    ],
                };
                sprites_to_draw
                    .into_iter()
                    .map(|(plane, sprite)| {
                        self.display.draw_16_16_sprite(
                            self.registers[&instruction.x()] as usize,
                            self.registers[&instruction.y()] as usize,
                            sprite.try_into().unwrap(),
                            plane,
                        )
                    })
                    .fold(false, |acc, is_pixel_erased| acc || is_pixel_erased)
            }
            _ => panic!("Unable to draw sprite.",),
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
            self.skip_next_instruction();
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
            self.skip_next_instruction();
        };
    }

    /// 0xF000 0xNNNN - Load `I` with a 16-bit address.
    fn load_i(&mut self) {
        let new_i_value = self.next_instruction().value();
        self.i_register.set(new_i_value);
    }

    /// 0xFX01 - Select zero or more drawing planes by bitmask (0 <= X <= 3).
    fn set_plane(&mut self, instruction: Instruction) {
        let plane = match instruction.x() {
            0 => return,
            1 => Plane::First,
            2 => Plane::Second,
            3 => Plane::Both,
            invalid_plane => panic!("Invalid plane to select {invalid_plane}."),
        };
        self.display.set_plane(plane);
    }

    /// 0xF002 - Store 16 bytes starting at `I` in the audio pattern buffer.
    fn load_audio_buffer(&mut self) {
        let buffer: [u8; 16] = self
            .memory
            .read_n_bytes(self.i_register.get(), 16)
            .try_into()
            .unwrap();
        self.audio_buffer = buffer;
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
        self.i_register.set(
            self.memory
                .get_font_address(register_x, ScreenResolution::Lores),
        );
    }

    /// Fx30 - Point I to 10-byte font sprite for digit VX (0..F)
    fn load_10_byte_font_to_i(&mut self, instruction: Instruction) {
        let register_x = self.registers[&instruction.x()];
        self.i_register.set(
            self.memory
                .get_font_address(register_x, ScreenResolution::Hires),
        );
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

    /// 0xFx3A - Set the audio pattern playback rate to 4000 * 2 ^ ((Vx - 64) / 48) Hz.
    fn set_pitch(&mut self, instruction: Instruction) {
        self.pitch = 4000 * 2u16.pow((self.registers[&instruction.x()] as u32 - 64) / 48);
    }

    /// Fx55 - LD [I], Vx
    /// Store registers V0 through Vx in memory starting at location `I`.
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
        if self.quirks.contains(&Quirks::IRegisterIncrementedWithX) {
            self.i_register
                .set(self.i_register.get() + instruction.x() as u16 + 1);
        }
    }

    /// Fx65 - LD Vx, [I]
    /// Read registers V0 through Vx from memory starting at location `I`.
    ///
    /// The interpreter reads values from memory starting at location `I` into
    /// registers V0 through Vx.
    fn ld_vx_i(&mut self, instruction: Instruction) {
        (0..=instruction.x()).for_each(|register| {
            self.registers.insert(
                register,
                self.memory.read(self.i_register.add(register as u16)),
            );
        });
        if self.quirks.contains(&Quirks::IRegisterIncrementedWithX) {
            self.i_register
                .set(self.i_register.get() + instruction.x() as u16 + 1);
        }
    }

    /// Fx75 - Store V0..VX in RPL user flags (x <= 7)
    fn load_rpl_flags(&mut self, instruction: Instruction) {
        let register_quantity = match self.mode {
            ChipMode::XOChip => &&instruction.x(),
            ChipMode::SuperChip if instruction.x() <= 7 => &&instruction.x(),
            _ => panic!(
                "Unable to load RPL {} flags on {} platform.",
                instruction.x(),
                self.mode
            ),
        };
        self.memory.write_rpl_flags(
            &self
                .registers
                .iter()
                .filter(|(i, _)| i < register_quantity)
                .map(|(i, _)| self.registers[i])
                .collect::<Vec<_>>(),
        );
    }

    /// Fx85 - Read V0..VX from RPL user flags (x <= 7)
    fn read_rpl_flags(&mut self, instruction: Instruction) {
        let register_quantity = match self.mode {
            ChipMode::XOChip => &&instruction.x(),
            ChipMode::SuperChip if instruction.x() <= 7 => &&instruction.x(),
            _ => panic!(
                "Unable to load RPL {} flags on {} platform.",
                instruction.x(),
                self.mode
            ),
        };
        self.memory
            .read_rpl_flags()
            .iter()
            .filter(|x| x < register_quantity)
            .enumerate()
            .for_each(|(i, &x)| {
                self.registers.insert(i as u8, x);
            });
    }

    fn skip_next_instruction(&mut self) {
        if self.mode == &ChipMode::XOChip {
            if self.next_instruction().nibbles() == (0xF, 0, 0, 0) {
                self.program_counter += 2;
            }
        } else {
            self.program_counter += 2;
        }
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
