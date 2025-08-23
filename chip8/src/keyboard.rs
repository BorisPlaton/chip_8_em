#[derive(Default)]
pub struct Keyboard {
    keys: [bool; 16],
}

impl Keyboard {
    pub fn is_pressed(&self, key: u8) -> bool {
        self.keys[key as usize]
    }

    pub fn pressed_key(&self) -> Option<u8> {
        self.keys
            .iter()
            .enumerate()
            .find_map(|(i, &key)| if key { Some(i as u8) } else { None })
    }
}
