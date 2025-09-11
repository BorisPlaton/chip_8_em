use crate::display::ScreenResolution;
use crate::platform::ChipMode;

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

    pub fn get_font_address(&self, digit: u8, resolution: ScreenResolution) -> u16 {
        match (self.mode, resolution, digit) {
            (_, ScreenResolution::Lores, _) if digit <= 0xF => (digit * 5) as u16,
            (ChipMode::SuperChip, ScreenResolution::Hires, _) if digit <= 9 => {
                (16 * 5 + digit * 10) as u16
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
                0xFF, 0xFF, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xFF, 0xFF, // 0
                0x18, 0x78, 0x78, 0x18, 0x18, 0x18, 0x18, 0x18, 0xFF, 0xFF, // 1
                0xFF, 0xFF, 0x03, 0x03, 0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, // 2
                0xFF, 0xFF, 0x03, 0x03, 0xFF, 0xFF, 0x03, 0x03, 0xFF, 0xFF, // 3
                0xC3, 0xC3, 0xC3, 0xC3, 0xFF, 0xFF, 0x03, 0x03, 0x03, 0x03, // 4
                0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, 0x03, 0x03, 0xFF, 0xFF, // 5
                0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, 0xC3, 0xC3, 0xFF, 0xFF, // 6
                0xFF, 0xFF, 0x03, 0x03, 0x06, 0x0C, 0x18, 0x18, 0x18, 0x18, // 7
                0xFF, 0xFF, 0xC3, 0xC3, 0xFF, 0xFF, 0xC3, 0xC3, 0xFF, 0xFF, // 8
                0xFF, 0xFF, 0xC3, 0xC3, 0xFF, 0xFF, 0x03, 0x03, 0xFF, 0xFF, // 9
                0x7E, 0xFF, 0xC3, 0xC3, 0xC3, 0xFF, 0xFf, 0xC3, 0xC3, 0xC3, // A
                0xFC, 0xFC, 0xC3, 0xC3, 0xFC, 0xFC, 0xC3, 0xC3, 0xFC, 0xFC, // B
                0x3C, 0xFF, 0xC3, 0xC0, 0xC0, 0xC0, 0xC0, 0xC3, 0xFF, 0x3C, // C
                0xFC, 0xFE, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xFE, 0xFC, // D
                0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, // E
                0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, 0xC0, 0xC0, 0xC0, 0xC0, // F
            ]);
        };

        font_sprites.into_iter().enumerate().for_each(|(i, val)| {
            self.map[i] = val;
        })
    }
}
