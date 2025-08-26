pub struct TimerRegister {
    value: u8,
}

impl Default for TimerRegister {
    fn default() -> Self {
        TimerRegister { value: 0 }
    }
}

impl TimerRegister {
    pub fn set(&mut self, value: u8) {
        self.value = value;
    }

    pub fn get(&mut self) -> u8 {
        self.value
    }

    pub fn tick(&mut self) {
        self.value = self.value.saturating_sub(1);
    }
}
