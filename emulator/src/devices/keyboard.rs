use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::{EventPump, Sdl};
use std::collections::HashMap;

pub struct KeyboardDevice {
    event_pump: EventPump,
    keymap: HashMap<Keycode, u8>,
}

impl KeyboardDevice {
    const NUM_1_CODE: u8 = 1;
    const NUM_2_CODE: u8 = 2;
    const NUM_3_CODE: u8 = 3;
    const NUM_4_CODE: u8 = 0xC;
    const NUM_Q_CODE: u8 = 4;
    const NUM_W_CODE: u8 = 5;
    const NUM_E_CODE: u8 = 6;
    const NUM_R_CODE: u8 = 0xD;
    const NUM_A_CODE: u8 = 7;
    const NUM_S_CODE: u8 = 8;
    const NUM_D_CODE: u8 = 9;
    const NUM_F_CODE: u8 = 0xE;
    const NUM_Z_CODE: u8 = 0xA;
    const NUM_X_CODE: u8 = 0;
    const NUM_C_CODE: u8 = 0xB;
    const NUM_V_CODE: u8 = 0xF;

    pub fn new(sdl_context: &Sdl) -> KeyboardDevice {
        let event_pump = sdl_context.event_pump().unwrap();
        let mut keymap = HashMap::new();

        keymap.insert(Keycode::NUM_1, Self::NUM_1_CODE);
        keymap.insert(Keycode::NUM_2, Self::NUM_2_CODE);
        keymap.insert(Keycode::NUM_3, Self::NUM_3_CODE);
        keymap.insert(Keycode::NUM_4, Self::NUM_4_CODE);

        keymap.insert(Keycode::Q, Self::NUM_Q_CODE);
        keymap.insert(Keycode::W, Self::NUM_W_CODE);
        keymap.insert(Keycode::E, Self::NUM_E_CODE);
        keymap.insert(Keycode::R, Self::NUM_R_CODE);

        keymap.insert(Keycode::A, Self::NUM_A_CODE);
        keymap.insert(Keycode::S, Self::NUM_S_CODE);
        keymap.insert(Keycode::D, Self::NUM_D_CODE);
        keymap.insert(Keycode::F, Self::NUM_F_CODE);

        keymap.insert(Keycode::Z, Self::NUM_Z_CODE);
        keymap.insert(Keycode::X, Self::NUM_X_CODE);
        keymap.insert(Keycode::C, Self::NUM_C_CODE);
        keymap.insert(Keycode::V, Self::NUM_V_CODE);

        KeyboardDevice { event_pump, keymap }
    }

    pub fn keys_state(&mut self) -> [bool; 16] {
        let mut keys_state = [false; 16];

        if let Some(_) = self
            .event_pump
            .poll_iter()
            .filter(|event| {
                if let Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } = event
                {
                    true
                } else {
                    false
                }
            })
            .next()
        {
            std::process::exit(0)
        }

        self.event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .for_each(|keycode| {
                if let Some(&index) = self.keymap.get(&keycode) {
                    keys_state[index as usize] = true;
                };
            });

        keys_state
    }
}
