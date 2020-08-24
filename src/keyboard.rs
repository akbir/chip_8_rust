
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

    pub fn key_press(&mut self, key: u8){
        self.set_key(key as usize, true)
    }

    pub fn key_release(&mut self, key: u8){
        self.set_key(key as usize, false)
    }


    fn set_key(&mut self, index: usize, state: bool) { self.keys[index] = state; }
}