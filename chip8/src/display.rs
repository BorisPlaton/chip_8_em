use crate::platform::Quirks;
use std::collections::HashSet;

type PixelErased = bool;

pub struct Display<'a> {
    first_plane: [bool; 8192],
    second_plane: [bool; 8192],
    is_hires: bool,
    current_plane: Plane,
    quirks: &'a HashSet<Quirks>,
}

#[derive(Clone, Copy)]
pub enum Plane {
    First,
    Second,
    Both,
}

pub enum ScreenResolution {
    Lores,
    Hires,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Color {
    Disabled,
    OnlyFirstPlane,
    OnlySecondPlane,
    Both,
}

impl<'a> Display<'a> {
    pub const WIDTH: usize = 64;
    pub const HEIGHT: usize = 32;

    pub const HIRES_WIDTH: usize = 128;
    pub const HIRES_HEIGHT: usize = 64;

    pub fn new(quirks: &'a HashSet<Quirks>) -> Self {
        Display {
            first_plane: [false; 8192],
            second_plane: [false; 8192],
            is_hires: false,
            current_plane: Plane::First,
            quirks,
        }
    }

    pub fn draw_sprite(
        &mut self,
        mut x: usize,
        mut y: usize,
        sprite: &[u8],
        plane: Plane,
    ) -> PixelErased {
        let mut pixel_erased = false;
        let screen_width = self.width();
        let screen_height = self.height();
        let wraps_instead_clipping = self.quirks.contains(&Quirks::WrapsInsteadClipping);
        let plane_map = match plane {
            Plane::First => &mut self.first_plane,
            Plane::Second => &mut self.second_plane,
            Plane::Both => panic!("Unable to write to both planes simultaneously."),
        };
        x %= screen_width;
        y %= screen_height;

        for row in 0..sprite.len() {
            let mut y_cord = y + row;

            if y_cord >= screen_height {
                if wraps_instead_clipping {
                    y_cord = y_cord - screen_height;
                } else {
                    break;
                }
            }

            for col in 0..8 {
                let mut x_cord = x + col;

                if x_cord >= screen_width {
                    if wraps_instead_clipping {
                        x_cord = x_cord - screen_width;
                    } else {
                        break;
                    }
                }

                let coord = x_cord + y_cord * screen_width;
                let is_current_pixel_set = plane_map[coord];
                let is_new_pixel_set = ((sprite[row] >> (7 - col)) & 1) == 1;
                plane_map[coord] ^= is_new_pixel_set;

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
        plane: Plane,
    ) -> PixelErased {
        let mut pixel_erased = false;
        let screen_width = self.width();
        let screen_height = self.height();
        let wraps_instead_clipping = self.quirks.contains(&Quirks::WrapsInsteadClipping);
        let plane_map = match plane {
            Plane::First => &mut self.first_plane,
            Plane::Second => &mut self.second_plane,
            Plane::Both => panic!("Unable to write to both planes simultaneously."),
        };
        x %= screen_width;
        y %= screen_height;

        for row in 0..16 {
            let mut y_cord = y + row;

            if y_cord >= screen_height {
                if wraps_instead_clipping {
                    y_cord = y_cord - screen_height;
                } else {
                    break;
                }
            }

            for col in 0..16 {
                let mut x_cord = x + col;

                if x_cord >= screen_width {
                    if wraps_instead_clipping {
                        x_cord = x_cord - screen_width;
                    } else {
                        break;
                    }
                }

                let coord = x_cord + y_cord * screen_width;
                let is_current_pixel_set = plane_map[coord];
                let is_new_pixel_set = ((sprite[row] >> (15 - col)) & 1) == 1;
                plane_map[coord] ^= is_new_pixel_set;

                if !pixel_erased && is_current_pixel_set && is_new_pixel_set {
                    pixel_erased = true;
                }
            }
        }

        pixel_erased
    }

    pub fn scroll_n_lines_down(&mut self, lines: u8) {
        let width = self.width();
        let height = self.height();
        let moved_part = lines as usize * width;
        let remaining_part = width * (height - lines as usize);
        self.get_selected_planes().into_iter().for_each(|plane| {
            plane.copy_within(..remaining_part, moved_part);
            plane[..moved_part].fill(false);
        });
    }

    pub fn scroll_n_lines_up(&mut self, lines: u8) {
        let width = self.width();
        let height = self.height();
        let moved_part = width * lines as usize;
        let remaining_part = width * (height - lines as usize);
        self.get_selected_planes().into_iter().for_each(|plane| {
            plane.copy_within(moved_part.., 0);
            plane[remaining_part..].fill(false);
        });
    }

    pub fn scroll_4_px_right(&mut self) {
        let width = self.width();
        let height = self.height();
        self.get_selected_planes().into_iter().for_each(|plane| {
            (0..height).into_iter().for_each(|row| {
                plane.copy_within(row * width..(row + 1) * width - 4, row * width + 4);
                plane[row * width..row * width + 4].copy_from_slice(&[false; 4]);
            });
        });
    }

    pub fn scroll_4_px_left(&mut self) {
        let width = self.width();
        let height = self.height();
        self.get_selected_planes().into_iter().for_each(|plane| {
            (0..height).into_iter().for_each(|row| {
                plane.copy_within(row * width + 4..(row + 1) * width, row * width);
                plane[row * width + width - 4..(row + 1) * width].copy_from_slice(&[false; 4]);
            });
        });
    }

    pub fn clear(&mut self) {
        self.get_selected_planes().into_iter().for_each(|plane| {
            plane.fill(false);
        });
    }

    pub fn set_plane(&mut self, plane: Plane) {
        self.current_plane = plane;
    }

    pub fn get_current_plane(&self) -> &Plane {
        &self.current_plane
    }

    pub fn display_bitplane(&self) -> [Color; 8192] {
        self.first_plane
            .iter()
            .zip(self.second_plane.iter())
            .map(|(first_plane_pixel, second_plane_pixel)| {
                match (first_plane_pixel, second_plane_pixel) {
                    (false, false) => Color::Disabled,
                    (true, false) => Color::OnlyFirstPlane,
                    (false, true) => Color::OnlySecondPlane,
                    (true, true) => Color::Both,
                }
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
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

    fn get_selected_planes(&mut self) -> Vec<&mut [bool; 8192]> {
        match self.current_plane {
            Plane::First => vec![&mut self.first_plane],
            Plane::Second => vec![&mut self.second_plane],
            Plane::Both => vec![&mut self.first_plane, &mut self.second_plane],
        }
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
