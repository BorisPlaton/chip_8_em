use sdl2::Sdl;
use sdl2::audio::{AudioCallback, AudioDevice as AudioDeviceSDL, AudioSpecDesired};

pub struct AudioDevice {
    subsystem: AudioDeviceSDL<SquareWave>,
}

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = self.volume * if self.phase < 0.5 { 1.0 } else { -1.0 };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

impl AudioDevice {
    pub fn new(sdl: &Sdl) -> AudioDevice {
        let audio_subsystem = sdl.audio().unwrap();
        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),
            samples: None,
        };

        let device = audio_subsystem
            .open_playback(None, &desired_spec, |spec| SquareWave {
                phase_inc: 240.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25,
            })
            .unwrap();

        AudioDevice { subsystem: device }
    }

    pub fn play_sound(&self, sound_register: u8) {
        if sound_register > 0 {
            self.subsystem.resume();
        } else {
            self.subsystem.pause();
        }
    }
}
