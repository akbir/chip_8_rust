mod display;

use processor::Processor;

mod processor;

fn main() {
    println!("Hello, world!");

    let mut my_chip8 = Processor::new();
    my_chip8.reset();
    my_chip8.execute_cycle()

}

