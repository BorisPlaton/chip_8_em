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
    const DISPLAY_WIDTH: usize = 63;
    const DISPLAY_HEIGHT: usize = 31;

    pub fn draw_sprite(&mut self, x: usize, y: usize, sprite: &[u8]) -> PixelErased {
        let mut pixel_erased = false;
        for row in 0..sprite.len() {
            for col in 0..8usize {
                let y_coord = self.wrap_coordinate(y + row, Self::DISPLAY_HEIGHT);
                let x_coord = self.wrap_coordinate(x + col, Self::DISPLAY_WIDTH);
                let is_display_pixel_set = self.buffer[x_coord + y_coord * Self::DISPLAY_WIDTH];
                let is_sprite_pixel_set = ((sprite[row] >> (7 - col)) & 1) == 1;
                self.buffer[x_coord + y_coord * Self::DISPLAY_WIDTH] ^= is_sprite_pixel_set;
                if !pixel_erased && is_display_pixel_set {
                    pixel_erased = is_display_pixel_set ^ is_sprite_pixel_set;
                }
            }
        }
        pixel_erased
    }

    pub fn clear(&mut self) {
        self.buffer = [false; 2048];
    }

    fn wrap_coordinate(&self, coordinate: usize, limit: usize) -> usize {
        if coordinate > limit {
            coordinate % limit - 1
        } else {
            coordinate
        }
    }
}
