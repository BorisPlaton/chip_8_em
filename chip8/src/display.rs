type PixelErased = bool;

pub struct Display {
    buffer: [bool; 8192],
    is_hires: bool,
}

pub enum ScreenResolution {
    Lores,
    Hires,
}

impl Default for Display {
    fn default() -> Display {
        Display {
            buffer: [false; 8192],
            is_hires: false,
        }
    }
}

impl Display {
    pub const WIDTH: usize = 64;
    pub const HEIGHT: usize = 32;

    pub const HIRES_WIDTH: usize = 128;
    pub const HIRES_HEIGHT: usize = 64;

    pub fn draw_sprite(&mut self, mut x: usize, mut y: usize, sprite: &[u8]) -> PixelErased {
        let mut pixel_erased = false;
        let screen_width = self.width();
        let screen_height = self.height();
        x %= screen_width;
        y %= screen_height;

        for row in 0..sprite.len() {
            let y_cord = y + row;

            if y_cord >= screen_height {
                break;
            }

            for col in 0..8 {
                let mut x_cord = x + col;

                if x_cord >= screen_width {
                    x_cord = x_cord - screen_width;
                }

                let coord = x_cord + y_cord * screen_width;
                let is_current_pixel_set = self.buffer[coord];
                let is_new_pixel_set = ((sprite[row] >> (7 - col)) & 1) == 1;
                self.buffer[coord] ^= is_new_pixel_set;

                if !pixel_erased && is_current_pixel_set && is_new_pixel_set {
                    pixel_erased = true;
                }
            }
        }

        pixel_erased
    }

    pub fn draw_16_16_sprite(
        &mut self,
        mut x: usize,
        mut y: usize,
        sprite: [u16; 16],
    ) -> PixelErased {
        let mut pixel_erased = false;
        let screen_width = self.width();
        let screen_height = self.height();
        x %= screen_width;
        y %= screen_height;

        for row in 0..16 {
            let y_cord = y + row;

            if y_cord >= screen_height {
                break;
            }

            for col in 0..16 {
                let mut x_cord = x + col;

                if x_cord >= screen_width {
                    x_cord = x_cord - screen_width;
                }

                let coord = x_cord + y_cord * screen_width;
                let is_current_pixel_set = self.buffer[coord];
                let is_new_pixel_set = ((sprite[row] >> (15 - col)) & 1) == 1;
                self.buffer[coord] ^= is_new_pixel_set;

                if !pixel_erased && is_current_pixel_set && is_new_pixel_set {
                    pixel_erased = true;
                }
            }
        }

        pixel_erased
    }

    pub fn buffer(&self) -> &[bool] {
        &self.buffer
    }

    pub fn clear(&mut self) {
        self.buffer.fill(false);
    }

    pub fn enable_hires(&mut self) {
        self.clear();
        self.is_hires = true;
    }

    pub fn disable_hires(&mut self) {
        self.clear();
        self.is_hires = false;
    }

    pub fn is_hires(&self) -> bool {
        self.is_hires
    }

    pub fn scroll_n_lines_down(&mut self, lines: u8) {
        let moved_part = lines as usize * self.width();
        let remaining_part = self.width() * (self.height() - lines as usize);
        self.buffer.copy_within(..remaining_part, moved_part);
        self.buffer[..moved_part].copy_from_slice(&vec![false; moved_part]);
    }

    pub fn scroll_4_px_right(&mut self) {
        (0..self.height()).into_iter().for_each(|row| {
            let width = self.width();
            self.buffer
                .copy_within(row * width..(row + 1) * width - 4, row * width + 4);
            self.buffer[row * width..row * width + 4].copy_from_slice(&[false; 4]);
        });
    }

    pub fn scroll_4_px_left(&mut self) {
        (0..self.height()).into_iter().for_each(|row| {
            let width = self.width();
            self.buffer
                .copy_within(row * width + 4..(row + 1) * width, row * width);
            self.buffer[row * width + width - 4..(row + 1) * width].copy_from_slice(&[false; 4]);
        });
    }

    pub fn width(&self) -> usize {
        if self.is_hires {
            Self::HIRES_WIDTH
        } else {
            Self::WIDTH
        }
    }

    pub fn height(&self) -> usize {
        if self.is_hires {
            Self::HIRES_HEIGHT
        } else {
            Self::HEIGHT
        }
    }
}
