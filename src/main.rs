mod chip;
mod game;
use std::{thread, time};
use std::thread::sleep;
use std::time::{Duration, Instant};
use fermium::timer::SDL_Delay;

fn main() {

    let chip = &mut chip::Chip::new();
    chip.load_rom();

    let target_ft = time::Duration::from_micros(1000000 / 60);
    unsafe {
        let mut game = game::Game::new();
        while(!game.quit){
            let pressed_keys= game.run();
            if chip.cpu.save_into_this_vx != 0x0 {
                    match pressed_keys.iter().position(|&s| s == 0x1){
                        Some(x)=> chip.cpu.v[chip.cpu.save_into_this_vx as usize] = x as u8,
                        None => {}
                    }
            }

            if pressed_keys.contains(&0x1) {
                chip.cpu.is_waiting_for_input = false;
            }
            let cycle_start = Instant::now();
            if chip.cpu.delay_timer > 0
            {
                chip.cpu.delay_timer -= 1;
            }

            if chip.cpu.sound_timer > 0
            {
                chip.cpu.sound_timer -= 1;
            }
            println!("looping");
            for i in 0..16 {
                if chip.cpu.is_waiting_for_input {
                    break;
                }
                chip.update(pressed_keys);
            }
            game.init();
            game.draw(&chip.cpu.display);
            game.commit();
            if let Some(i) = (target_ft).checked_sub(cycle_start.elapsed()) {
                SDL_Delay(i.as_millis() as u32);
            }
        }
    }
    return;
}
