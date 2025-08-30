type PixelErased = bool;

pub struct Display {
    buffer: [bool; 2048],
}

impl Default for Display {
    fn default() -> Display {
        Display {
            buffer: [false; 2048],
        }
    }
}

impl Display {
    pub const WIDTH: usize = 64;
    pub const HEIGHT: usize = 32;

    pub fn draw_sprite(&mut self, mut x: usize, mut y: usize, sprite: &[u8]) -> PixelErased {
        let mut pixel_erased = false;
        x %= Self::WIDTH;
        y %= Self::HEIGHT;

        for row in 0..sprite.len() {
            let y_cord = y + row;

            if y_cord >= Self::HEIGHT {
                break;
            }

            for col in 0..8 {
                let x_cord = x + col;

                if x_cord >= Self::WIDTH {
                    break;
                }

                let coord = x_cord + y_cord * Self::WIDTH;
                let is_display_pixel_set = self.buffer[coord];
                self.buffer[coord] ^= ((sprite[row] >> (7 - col)) & 1) == 1;

                if !pixel_erased && is_display_pixel_set {
                    pixel_erased = !self.buffer[coord];
                }
            }
        }
        pixel_erased
    }

    pub fn buffer(&self) -> &[bool] {
        &self.buffer
    }

    pub fn clear(&mut self) {
        self.buffer = [false; 2048];
    }
}
