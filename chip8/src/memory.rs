use crate::modes::ChipMode;

// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.1
//
// Memory Map:
// +---------------+= 0xFFF (4095) End of Chip-8 RAM
// |               |
// |               |
// |               |
// |               |
// |               |
// | 0x200 to 0xFFF|
// |     Chip-8    |
// | Program / Data|
// |     Space     |
// |               |
// |               |
// |               |
// +- - - - - - - -+= 0x600 (1536) Start of ETI 660 Chip-8 programs
// |               |
// |               |
// |               |
// +---------------+= 0x200 (512) Start of most Chip-8 programs
// | 0x000 to 0x1FF|
// | Reserved for  |
// |  interpreter  |
// +---------------+= 0x000 (0) Start of Chip-8 RAM
pub struct Memory<'a> {
    map: [u8; 4096],
    mode: &'a ChipMode,
    rpl_flags: [u8; 8],
}

pub enum FontSize {
    Standard,
    Extended,
}

impl<'a> Memory<'a> {
    const RESERVED_ADDR_START: u16 = 0;
    pub const PROGRAM_ADDR_START: u16 = 0x200;
    pub const MEMORY_SIZE: u16 = 0xFFF;

    pub fn new(program: &[u8], mode: &'a ChipMode) -> Memory<'a> {
        let mut memory = Memory {
            map: [0; 4096],
            rpl_flags: [0; 8],
            mode,
        };

        memory.load_font_sprites();

        program.iter().enumerate().for_each(|(i, &byte)| {
            memory.map[Self::PROGRAM_ADDR_START as usize + i] = byte;
        });

        memory
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            Memory::RESERVED_ADDR_START..Memory::PROGRAM_ADDR_START => {
                panic!(
                    "Attempted to write to CHIP-8 interpreter address space: {:04x}",
                    addr
                );
            }
            Memory::PROGRAM_ADDR_START..=Memory::MEMORY_SIZE => self.map[addr as usize] = val,
            _ => panic!(
                "Attempted to write to the out-of-bound address: {:04x}",
                addr
            ),
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        if addr > Self::MEMORY_SIZE {
            panic!("Attempted to read out-of-bound address: {:04x}", addr);
        }
        self.map[addr as usize]
    }

    pub fn get_font_address(&self, digit: u8, size: FontSize) -> u16 {
        match (self.mode, size, digit) {
            (_, FontSize::Standard, _) if digit <= 0xF => (digit * 5) as u16,
            (ChipMode::SuperChip, FontSize::Extended, _) if digit <= 9 => {
                (0xF * 5 + digit * 10) as u16
            }
            _ => panic!("Invalid font sprite {digit} for mode {}", self.mode),
        }
    }

    pub fn write_rpl_flags(&mut self, flags: &[u8]) {
        flags.iter().enumerate().for_each(|(i, &flag)| {
            self.rpl_flags[i] = flag;
        });
    }

    pub fn read_rpl_flags(&mut self) -> &[u8] {
        &self.rpl_flags
    }

    fn load_font_sprites(&mut self) {
        let mut font_sprites = vec![];

        font_sprites.extend_from_slice(&[
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ]);

        if self.mode == &ChipMode::SuperChip {
            font_sprites.extend_from_slice(&[
                0xFC, 0x84, 0x84, 0x84, 0x84, 0x84, 0x84, 0x84, 0xFC, 0x00, // 0
                0x18, 0x38, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00, // 1
                0xFC, 0x84, 0x04, 0x04, 0xFC, 0x80, 0x80, 0x80, 0xFC, 0x00, // 2
                0xFC, 0x84, 0x04, 0x04, 0x3C, 0x04, 0x04, 0x84, 0xFC, 0x00, // 3
                0x84, 0x84, 0x84, 0x84, 0xFC, 0x04, 0x04, 0x04, 0x04, 0x00, // 4
                0xFC, 0x80, 0x80, 0x80, 0xFC, 0x04, 0x04, 0x04, 0xFC, 0x00, // 5
                0xFC, 0x80, 0x80, 0x80, 0xFC, 0x84, 0x84, 0x84, 0xFC, 0x00, // 6
                0xFC, 0x84, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x00, // 7
                0xFC, 0x84, 0x84, 0x84, 0xFC, 0x84, 0x84, 0x84, 0xFC, 0x00, // 8
                0xFC, 0x84, 0x84, 0x84, 0xFC, 0x04, 0x04, 0x04, 0xFC, 0x00, // 9
            ]);
        };

        font_sprites.into_iter().enumerate().for_each(|(i, val)| {
            self.map[i] = val;
        })
    }
}
