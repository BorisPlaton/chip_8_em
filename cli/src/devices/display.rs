use chip8::display::{Color, Display};
use sdl2::Sdl;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use std::collections::HashMap;

pub struct DisplayDevice {
    texture_creator: TextureCreator<WindowContext>,
    current_frame: Frame,
    canvas: WindowCanvas,
    width: u32,
    height: u32,
    palette: HashMap<Color, (u8, u8, u8)>,
}

struct Frame {
    pixels: [u8; 24576],
}

impl DisplayDevice {
    pub fn new(
        sdl_context: &Sdl,
        title: &str,
        width: u32,
        height: u32,
        scale: u32,
        palette: HashMap<Color, (u8, u8, u8)>,
    ) -> DisplayDevice {
        let window = sdl_context
            .video()
            .unwrap()
            .window(title, width * scale, height * scale)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().present_vsync().build().unwrap();
        canvas.set_scale(scale as f32, scale as f32).unwrap();
        let texture_creator = canvas.texture_creator();

        DisplayDevice {
            texture_creator,
            width,
            height,
            canvas,
            palette,
            current_frame: Frame::default(),
        }
    }

    pub fn draw(&mut self, display: &Display) {
        self.height = display.height() as u32;
        self.width = display.width() as u32;

        let mut texture = self
            .texture_creator
            .create_texture_target(PixelFormatEnum::RGB24, self.width, self.height)
            .unwrap();

        self.current_frame.update(display, &self.palette);
        texture
            .update(None, self.current_frame.pixels(), (self.width * 3) as usize)
            .unwrap();

        self.canvas.copy(&texture, None, None).unwrap();
        self.canvas.present();
    }
}

impl Frame {
    fn update(&mut self, display: &Display, palette: &HashMap<Color, (u8, u8, u8)>) {
        display
            .display_bitplane()
            .iter()
            .enumerate()
            .for_each(|(pixel, color)| {
                let rgb = &palette[color];
                self.pixels[pixel * 3] = rgb.0;
                self.pixels[pixel * 3 + 1] = rgb.1;
                self.pixels[pixel * 3 + 2] = rgb.2;
            });
    }

    fn pixels(&self) -> &[u8] {
        &self.pixels
    }
}

impl Default for Frame {
    fn default() -> Self {
        Frame { pixels: [0; 24576] }
    }
}
