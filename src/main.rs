mod chip;
mod game;
use std::{thread, time};
use std::thread::sleep;
use std::time::{Duration, Instant};
use fermium::timer::SDL_Delay;

fn main() {
    println!("Hello, world!");

    let chip = &mut chip::Chip::new();
    chip.load_rom();

    let target_ft = time::Duration::from_micros(1000000 / 60);
    unsafe {
        let mut game = game::Game::new();
        while(!game.quit){
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
                chip.update();
            }
            game.init();
            game.draw(&chip.cpu.display);
            game.commit();
            game.run();
            if let Some(i) = (target_ft).checked_sub(cycle_start.elapsed()) {
                thread::sleep(i)
            }
        }
    }
    return;
}
