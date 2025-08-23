use std::time::Instant;

pub struct TimerRegister {
    value: u8,
    set_at: Instant,
}

impl Default for TimerRegister {
    fn default() -> Self {
        TimerRegister {
            value: 0,
            set_at: Instant::now(),
        }
    }
}

impl TimerRegister {
    const DECREASE_FREQUENCY: u8 = 60;

    pub fn set(&mut self, value: u8) {
        self.value = value;
        self.set_at = Instant::now();
    }

    pub fn get(&mut self) -> u8 {
        if self.value == 0 {
            return 0;
        }

        let elapsed_duration = self.set_at.elapsed();
        let elapsed_seconds =
            elapsed_duration.as_secs() as f32 + elapsed_duration.subsec_millis() as f32 / 1_000.0;
        let decreased_value = (elapsed_seconds * Self::DECREASE_FREQUENCY as f32) as u32;

        self.value = if decreased_value >= self.value as u32 {
            0
        } else {
            self.value - decreased_value as u8
        };

        self.value
    }
}
