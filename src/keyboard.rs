use sdl::event::Key;

// copied from https://github.com/mikezaby/chip-8.rs


pub struct Keyboard {
    keys: [bool; 16]
}

impl Keyboard{
    pub fn new()-> Keyboard {
        Keyboard{keys: [false; 16]}
    }

    pub fn pressed(&mut self, index: usize) -> bool {
        self.keys[index]
    }

    pub fn clear(&mut self) {
        self.keys = [false; 16]
    }

    pub fn press(&mut self, key: Key, state: bool) {
        match key {
            Key::Num1 => self.set_key(0x1, state),
            Key::Num2 => self.set_key(0x2, state),
            Key::Num3 => self.set_key(0x3, state),
            Key::Num4 => self.set_key(0xc, state),
            Key::Q => self.set_key(0x4, state),
            Key::W => self.set_key(0x5, state),
            Key::E => self.set_key(0x6, state),
            Key::R => self.set_key(0xd, state),
            Key::A => self.set_key(0x7, state),
            Key::S => self.set_key(0x8, state),
            Key::D => self.set_key(0x9, state),
            Key::F => self.set_key(0xe, state),
            Key::Z => self.set_key(0xa, state),
            Key::X => self.set_key(0x0, state),
            Key::C => self.set_key(0xb, state),
            Key::V => self.set_key(0xf, state),
            _ => ()
        }
    }

    fn set_key(&mut self, index: usize, state: bool) { self.keys[index] = state; }
}