use sdl2::Sdl;
use sdl2::audio::{AudioCallback, AudioDevice as AudioDeviceSDL, AudioSpecDesired};

pub struct AudioDevice {
    subsystem: AudioDeviceSDL<ChipAudio>,
}

struct ChipAudio {
    pattern: [u8; 16],
    pitch: u16,
    phase: f64,
    sample_rate: f64,
}

impl AudioCallback for ChipAudio {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for sample in out.iter_mut() {
            let pattern_index = (self.phase / 8.0).floor() as usize % 16;
            let current_byte = self.pattern[pattern_index];
            let bit_value = (current_byte >> (7 - (self.phase as usize % 8))) & 1;

            *sample = if bit_value == 1 { 0.5 } else { -0.5 };

            self.phase += (self.pitch as f64) / self.sample_rate * 128.0;
            if self.phase >= 128.0 {
                self.phase -= 128.0;
            }
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
            .open_playback(None, &desired_spec, |spec| ChipAudio {
                pattern: [0xFF; 16],
                phase: 0.0,
                sample_rate: spec.freq as f64,
                pitch: 0,
            })
            .unwrap();

        AudioDevice { subsystem: device }
    }

    pub fn configure(&mut self, audio_buffer: &[u8], pitch: u16) {
        let mut audio_lock = self.subsystem.lock();
        audio_lock.pattern.copy_from_slice(audio_buffer);
        audio_lock.pitch = pitch;
    }

    pub fn play_sound(&mut self, sound_register: u8, audio_buffer: &[u8], pitch: u16) {
        if sound_register > 0 {
            self.configure(audio_buffer, pitch);
            self.subsystem.resume();
        } else {
            self.subsystem.pause();
        }
    }
}
