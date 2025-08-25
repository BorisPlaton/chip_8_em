#[derive(Default)]
pub struct Keyboard {
    keys: [bool; 16],
}

impl Keyboard {
    pub fn press_key(&mut self, key: u8) {
        self.keys[key as usize] = true;
    }

    pub fn release_key(&mut self, key: u8) {
        self.keys[key as usize] = false;
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        self.keys[key as usize]
    }

    pub fn pressed_key(&self) -> Option<u8> {
        self.keys
            .iter()
            .enumerate()
            .find_map(|(i, &key)| if key { Some(i as u8) } else { None })
    }
}
