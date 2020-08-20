use processor::Processor;
use sdl;

mod processor;
mod display;
mod keyboard;

fn main() {
    let mut my_chip8 = Processor::new();
    my_chip8.reset();

    sdl::init(&[sdl::InitFlag::Video, sdl::InitFlag::Audio, sdl::InitFlag::Timer]);
    my_chip8.execute_cycle();


    sdl::quit();
}

