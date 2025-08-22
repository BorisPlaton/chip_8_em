#[derive(Default)]
pub struct Keyboard {
    keys: [bool; 16],
}

impl Keyboard {
    pub fn is_pressed(&self, key: u8) -> bool {
        self.keys[key as usize]
    }
}
