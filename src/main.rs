use processor::Processor;
extern crate piston_window;

use piston_window::*;

mod processor;
mod display;
mod keyboard;

const SCALE: usize = 20;

fn main() {
    let mut my_chip8 = Processor::new();
    my_chip8.reset();
    my_chip8.load_rom("roms/pong");

    let mut window: PistonWindow = WindowSettings::new(
        "Chip 8 Emulator!",
        [(display::WIDTH * SCALE) as u32, (display::HEIGHT * SCALE) as u32])
        .exit_on_esc(true)
        .build()
        .unwrap();


    //start game
    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            draw_screen(&my_chip8.display.get_buffer(), &mut window, &e);
        }
        if let Some(u) = e.update_args() {
            my_chip8.execute_cycle();
        }

        if let Some(Button::Keyboard(key)) = e.release_args() {
            if let Some(key_value) = key_value(&key) {
                my_chip8.keyboard.key_release(key_value);
            }
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            if let Some(key_value) = key_value(&key) {
                my_chip8.keyboard.key_press(key_value);
            }
        }
    }

    fn key_value(key: &Key) -> Option<u8> {
        if key.code() >= 48 && key.code() <= 57 {
            Some((key.code() - 48) as u8)
        } else if key.code() >= 97 && key.code() <= 102 {
            Some((key.code() - 97 + 10) as u8)
        } else {
            None
        }
    }

    fn draw_screen(display_buffer: &display::Buffer, window: &mut PistonWindow, event: &Event){
        window.draw_2d(event, |context, graphics, _d| {
            piston_window::clear(color::BLACK, graphics);
            for (i, row) in display_buffer.iter().enumerate() {
                for (j, val) in row.iter().enumerate() {
                    if *val {
                        let dimensions = [(j * SCALE) as f64, (i * SCALE) as f64, SCALE as f64, SCALE as f64];
                        Rectangle::new(color::WHITE)
                            .draw(dimensions, &context.draw_state, context.transform, graphics);
                    }
                }
            }
        });
    }
}
