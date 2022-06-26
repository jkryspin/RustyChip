mod chip;
mod game;

fn main() {
    println!("Hello, world!");

    let chip = &mut chip::Chip::new();
    chip.load_rom();

    for i in 0..10000 {
        chip.update();
    }
    unsafe { game::main(); }
    return;
}
