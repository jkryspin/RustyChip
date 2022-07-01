mod chip;
mod files;
mod game;
mod input;
mod sounds;

use crate::files::{file_names, get_contents};
use crate::input::get_user_selection;
use fermium::timer::SDL_Delay;
use std::ffi::OsString;
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::{fs, thread, time};

fn main() {
    let files = file_names();
    let rom_selected = get_user_selection(files);
    let contents = get_contents(rom_selected);

    let mut chip =  chip::Chip::new(contents);

    let sounds = sounds::Sound::new();
    // 60 updates per second
    // 1 update = 1 seconds /60 or 16.666 hz
    let target_frame_time = time::Duration::from_micros(1000000 / 60);
    unsafe {
        let mut game = game::Game::new();
        while !game.should_quit {
            let cycle_start = Instant::now();
            let pressed_keys = game.run();

            if chip.cpu.save_into_this_vx != 0x0 {
                if let Some(x) = pressed_keys.iter().position(|&s| s == 0x1) {
                    chip.cpu.v[chip.cpu.save_into_this_vx as usize] = x as u8;
                }
            }

            if pressed_keys.contains(&0x1) {
                chip.cpu.is_waiting_for_input = false;
            }

            if chip.cpu.delay_timer > 0 {
                chip.cpu.delay_timer -= 1;
            }

            if chip.cpu.sound_timer > 0 {
                sounds.play_noise();
                chip.cpu.sound_timer -= 1;
            }

            for _ in 0..16 {
                if chip.cpu.is_waiting_for_input {
                    break;
                }
                chip.update(pressed_keys);
            }
            game.init();
            game.draw(&chip.cpu.display);
            game.commit();
            if let Some(i) = (target_frame_time).checked_sub(cycle_start.elapsed()) {
                std::thread::sleep(i);
            }
        }
    }
}
