use chip8::display::Display;

pub struct Frame {
    pixels: [u8; 6144],
}

impl Default for Frame {
    fn default() -> Self {
        Frame { pixels: [0; 6144] }
    }
}

impl Frame {
    pub fn update(&mut self, display: &Display) {
        display
            .buffer()
            .iter()
            .enumerate()
            .for_each(|(pixel, &is_enabled)| {
                self.pixels[pixel * 3] = 64;
                self.pixels[pixel * 3 + 1] = if is_enabled { 128 } else { 0 };
                self.pixels[pixel * 3 + 2] = 128;
            });
    }

    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }
}
