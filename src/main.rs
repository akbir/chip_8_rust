use processor::Processor;
use sdl;
use sdl::event::Event;


mod processor;
mod display;
mod keyboard;

fn main() {
    let mut my_chip8 = Processor::new();
    my_chip8.reset();
    my_chip8.load_rom("roms/pong");

    sdl::init(&[sdl::InitFlag::Video, sdl::InitFlag::Audio, sdl::InitFlag::Timer]);
    'main : loop {
        'event : loop {
            match sdl::event::poll_event() {
                Event::Quit                  => break 'main,
                Event::None                  => break 'event,
                Event::Key(key, state, _, _) => my_chip8.keyboard.press(key, state),
                _                            => {}
            }
        }
        my_chip8.execute_cycle();
        my_chip8.display.draw_screen();
    }

    sdl::quit();

}

