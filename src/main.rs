mod chip;
mod game;
use std::{thread, time};

fn main() {
    println!("Hello, world!");

    let chip = &mut chip::Chip::new();
    chip.load_rom();

    unsafe {
        let game = game::Game::new();
        game.init();
        let mut i = 0;
        while(true && i< 100000){
            chip.update();
            if &i%10000 == 0 {
                game.init();
                game.draw(&chip.cpu.display);
                game.commit();
                println!("{}", i);
            }

            i += 1;
        }
        // game.add_rect(0);
        game.run();
    }
    return;
}
