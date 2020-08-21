use rand::Rng;
use crate::display::Display;
use crate::keyboard::Keyboard;

pub struct Processor {
    // storage
    memory: [u8; 4096],
    register: [u8; 16],
    stack: [u16; 16],

    // counters
    program_counter: u16,
    index_register: u16,
    stack_pointer: u8,

    // timers
    sound_timer: u8,
    delay_timer: u8,

    // hardware
    pub display : Display,
    pub keyboard : Keyboard,

}

fn read_word(memory: [u8; 4096], index: u16) -> u16 {
    // Apply XOR to index and index +1
    (memory[index as usize] as u16) << 8
        | (memory[(index + 1) as usize] as u16)
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            memory: [0; 4096],
            register: [0; 16],
            stack: [0; 16],
            program_counter: 0,
            index_register: 0,
            stack_pointer: 0,
            sound_timer: 0,
            delay_timer: 0,
            keyboard: Keyboard::new(),
            display: Display::new()
        }
    }

    pub fn reset(&mut self) {
        // first 512 bits reserved
        self.program_counter = 0x200;
        self.index_register = 0;
        self.stack_pointer = 0;
        self.sound_timer = 0;
        self.delay_timer = 0;

        // clear display
        self.display.clear();
        // clear keyboard
        self.keyboard.clear();
        // set reserved memory
        self.memory[ 0 .. 80].copy_from_slice(&FONT_SET);
    }

    pub fn execute_cycle(&mut self) {
        // fetch opcode
        let _opcode = read_word(self.memory, self.index_register);

        // execute opcode
        self.execute_opcode(_opcode);

        // update timers
        self.decrement_delay_timer();
        self.decrement_sound_timer();
    }

    fn execute_opcode(&mut self, opcode: u16) {
        // break up into nibbles
        let op_1 = (opcode & 0xF000) >> 12;
        let op_2 = (opcode & 0x0F00) >> 8;
        let op_3 = (opcode & 0x00F0) >> 4;
        let op_4 = opcode & 0x000F;

        // helper addresses
        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;
        let n = opcode & 0x000F;

        // check registers
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let vx = self.register[x];
        let vy = self.register[y];

        // we read the opcode so move program counter forward
        self.program_counter += 2;

        match (op_1, op_2, op_3, op_4) {
            // Clear Screen
            (0, 0, 0xE, 0) => self.display.clear(),

            // Return from subroutine
            (0, 0, 0xE, 0xE) => {
                self.stack_pointer -= 1;
                self.program_counter = self.stack[self.stack_pointer as usize]
            },

            // Jump to Address NNN
            (0x1, _, _, _) => self.program_counter = nnn,

            // Call subroutine
            (0x2, _, _, _) => {
                self.stack[self.stack_pointer as usize] = self.program_counter;
                self.stack_pointer += 1;
                self.program_counter = nnn;
            },

            // Skips the next instruction if VX equals NN
            (0x3, _, _, _) => self.program_counter += if vx == nn { 2 } else { 0 },

            // Skips the next instruction if VX doesn't equals NN
            (0x4, _, _, _) => self.program_counter += if vx != nn { 2 } else { 0 },

            // Skips the next instruction if VX equals VY
            (0x5, _, _, _) => self.program_counter += if vx == vy { 2 } else { 0 },

            // Sets VX to NN
            (0x6, _, _, _) => self.register[x] = nn,

            // Adds NN to VX
            (0x7, _, _, _) => self.register[x] += nn,

            // Sets VX to the value of VY.
            (0x8, _, _, 0x0) => self.register[x] = vy,

            // Sets VX to VX or VY (Bitwise OR operation)
            (0x8, _, _, 0x1) => self.register[x] = vx | vy,

            // Sets VX to VX and VY (Bitwise AND operation)
            (0x8, _, _, 0x2) => self.register[x] = vx & vy,

            // Sets VX to VX xor VY
            (0x8, _, _, 0x3) => self.register[x] = vx ^ vy,

            // Adds VY to VX. VF is set to 1 if there's a carry
            (0x8, _, _, 0x4) => {
                let (res, overflow) = vx.overflowing_add(vy);
                self.register[0xF] = overflow as u8;
                self.register[x] = res;
            }

            // Subtract VY from VX. VF is set to 0 if there's a borrow
            (0x8, _, _, 0x5) => {
                let (res, overflow) = vx.overflowing_sub(vy);
                self.register[0xF] = !overflow as u8;
                self.register[x] = res;
            }

            // Store least significant bit of VX in VF and shifts VX to the right by 1
            (0x8, _, _, 0x6) => {
                self.register[0xF] = vx & 0x1;
                self.register[x] >>= 1;
            }

            // Sets VX to VY minus VX. VF is set to 0 when there's a borrow
            (0x8, _, _, 0x7) => {
                let (res, overflow) = vy.overflowing_sub(vx);
                self.register[0xF] = !overflow as u8;
                self.register[x] = res;
            }

            // Stores the most significant bit of VX in VF and then shifts VX to the left by 1
            (0x8, _, _, 0xE) => {
                self.register[0xF] = vx & 0x80;
                self.register[x] <<= 1;
            }

            // Skips the next instruction if VX doesn't equal VY
            (0x9, _, _, 0xE) => self.program_counter += if vx != vy { 2 } else { 0 },

            // Sets I to the address NNN
            (0xA, _, _, _) => self.index_register = nnn,

            // Jumps to the address NNN plus V0.
            (0xB, _, _, _) => self.index_register = nnn + self.register[0] as u16,

            // Set VX to random number and NN
            (0xC, _, _, _) => self.register[x] = nn & rand::thread_rng().gen_range(1, 255),

            // Draws a sprite at coordinate (VX, VY), set VF to 1 if pixels unset else 0
            (0xD, _, _, _) => {
                let sprite = &self.memory
                    [self.index_register as usize .. (self.index_register + n as u16) as usize];
                self.register[0xF] = self.display.draw(vx as usize, vy as usize, sprite)
            },

            // Skips the next instruction if the key stored in VX is pressed
            (0xE, _, 0x9, 0xE) => self.program_counter += if self.keyboard.pressed(vx as usize) { 2 } else { 0 },

            // Skips the next instruction if the key stored in VX isn't pressed
            (0xE, _, 0xA, 0x1) => self.program_counter += if self.keyboard.pressed(vx as usize) { 0 } else { 2 },

            // Sets VX to the value of the delay timer
            (0xF, _, 0x0, 0x7) => self.register[x] = self.delay_timer,

            // A key press is awaited, and then stored in VX
            (0xF, _, 0x0, 0xA) => {
                // move back register (no key is pressed)
                self.program_counter -= 2;

                for key in 0 .. 0xF{
                    if self.keyboard.pressed(key){
                        self.register[x] = key as u8;
                        self.program_counter += 2;
                    }
                }
            },

            // Sets the delay timer to VX
            (0xF, _, 0x1, 0x5) => self.delay_timer = vx,

            // Sets the sound timer to VX
            (0xF, _, 0x1, 0x8) => self.sound_timer = vx,

            // Adds VX to I
            (0xF, _, 0x1, 0xE) => self.index_register += vx as u16,

            // Sets I to the location of the sprite for the character in VX
            (0xF, _, 0x2, 0x9) => self.index_register = (vx * 5) as u16,

            // Set the decimal rep of VX to memory
            (0xF, _, 0x3, 0x3) => {
                self.memory[self.index_register as usize] = vx / 100;
                self.memory[self.index_register as usize + 1] = (vx / 10) % 10;
                self.memory[self.index_register as usize + 2] = (vx % 100) % 10;
            }

            // Stores V0 to VX (including VX) in memory starting at address I
            (0xF, _, 0x5, 0x5) => self.memory
                [self.index_register as usize .. (x + 1 + self.index_register as usize)]
                .copy_from_slice(&self.register[0..x + 1]),

            // Fills V0 to VX (including VX) with values from memory starting at address I
            (0xF, _, 0x6, 0x5) => self.register[0..x + 1]
                .copy_from_slice(&self.memory
                    [self.index_register as usize .. (x + 1 + self.index_register as usize)]),

            // ...
            (_, _, _, _) => ()
        }
}


    fn decrement_delay_timer(&mut self) {
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }
        }
    fn decrement_sound_timer(&mut self) {
        if self.sound_timer > 0{
            self.sound_timer -=1;
        }
    }
}
static FONT_SET: [u8; 80] = [0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70,
0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0, 0x10, 0xF0, 0x10, 0xF0,
0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0,
0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40,
0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0, 0x10, 0xF0,
0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0,
0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0,
0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80];
