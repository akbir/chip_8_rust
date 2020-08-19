pub struct Display {
    gfx : [[u8; 64]; 32],
    draw_flag: bool,
}

impl Display{
    pub fn new() -> Display {
        Display{
            gfx: [[0;64]; 32],
            draw_flag: true
        }
    }

    pub fn clear(&mut self) {
        self.gfx = [[0; 64]; 32];
        self.draw_flag = true;
    }

     pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> u8 {
        let mut collision = 0u8;
        let n = sprite.len() as usize;
        let mut yj: usize;
        let mut xi: usize;

        for j in range(0, n) {
            for i in range(0, 8) {
                yj = (y + j) % 32;
                xi = (x + i) % 64;

                if (sprite[j] & (0x80 >> i)) != 0 {
                    if self.gfx[yj][xi] == 1 { collision = 1 }
                    self.gfx[yj][xi] ^= 1;
                }
            }
        }

        self.draw_flag = true;
        collision
    }
}