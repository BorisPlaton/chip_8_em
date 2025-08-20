pub struct Display {
    buffer: [bool; 256],
}

impl Default for Display {
    fn default() -> Display {
        Display {
            buffer: [false; 256],
        }
    }
}

impl Display {
    const DISPLAY_WIDTH: u8 = 0x3F;
    const DISPLAY_HEIGHT: u8 = 0x1F;

    pub fn set_pixel(&mut self, x: u8, y: u8, pixel: bool) {
        if (x > Self::DISPLAY_WIDTH) || (y >= Self::DISPLAY_HEIGHT) {
            panic!("Invalid pixel coordinates: ({x}, {y})");
        }
        self.buffer[(x + (y * Self::DISPLAY_WIDTH)) as usize] = pixel;
    }

    pub fn clear(&mut self) {
        self.buffer = [false; 256];
    }
}
