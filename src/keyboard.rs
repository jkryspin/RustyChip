use fermium::keycode::*;
use std::collections::HashMap;

pub struct Keyboard {
    pub pressed_keys: [u8; 16],
    key_map: HashMap<SDL_Keycode, u8>,
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {
            pressed_keys: [0; 16],
            key_map: Keyboard::key_map(),
        }
    }

    fn set_pressed_keys(&mut self, down: bool, index: u8) {
        self.pressed_keys[index as usize] = down as u8
    }

    pub fn set_pressed_from_scancode(&mut self, i: SDL_Keycode, is_down: bool) {
        if let Some(key) = self.key_map.get(&i) {
            self.set_pressed_keys(is_down, *key);
        }
    }

    fn key_map() -> HashMap<SDL_Keycode, u8> {
        let mut scan_to_index: HashMap<SDL_Keycode, u8> = HashMap::new();

        scan_to_index.insert(SDLK_1, 0x1);
        scan_to_index.insert(SDLK_2, 0x2);
        scan_to_index.insert(SDLK_3, 0x3);
        scan_to_index.insert(SDLK_4, 0xc);
        scan_to_index.insert(SDLK_q, 0x4);
        scan_to_index.insert(SDLK_w, 0x5);
        scan_to_index.insert(SDLK_e, 0x6);
        scan_to_index.insert(SDLK_r, 0xD);
        scan_to_index.insert(SDLK_a, 0x7);
        scan_to_index.insert(SDLK_s, 0x8);
        scan_to_index.insert(SDLK_d, 0x9);
        scan_to_index.insert(SDLK_f, 0xE);
        scan_to_index.insert(SDLK_z, 0xA);
        scan_to_index.insert(SDLK_x, 0x0);
        scan_to_index.insert(SDLK_c, 0xB);
        scan_to_index.insert(SDLK_v, 0xF);
        return scan_to_index;
    }
}
