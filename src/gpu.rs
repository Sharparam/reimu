pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
const SCREEN_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

pub struct Gpu {
    screen: [bool; SCREEN_SIZE],
}

impl Gpu {
    pub fn new() -> Self {
        Self {
            screen: [false; SCREEN_SIZE],
        }
    }

    pub fn screen(&self) -> &[bool] {
        &self.screen
    }

    pub fn clear(&mut self) {
        self.screen.fill(false);
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) -> bool {
        let adj_x = x % SCREEN_WIDTH;
        let adj_y = y % SCREEN_HEIGHT;
        let idx = adj_y * SCREEN_WIDTH + adj_x;
        let current = self.screen[idx];
        let hit = current && value;
        self.screen[idx] ^= value;
        hit
    }

    pub fn draw_sprite(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        let mut hit = false;

        for (row, byte) in sprite.iter().enumerate() {
            for col in 0..8 {
                let mask = 1 << col;
                let masked = byte & mask;
                let value = masked == mask;
                let adj_col = 7 - col;
                hit |= self.set(x + adj_col, y + row, value);
            }
        }

        hit
    }

    #[allow(dead_code)]
    pub fn dump(&self) {
        const FULL: char = '█';
        const UPPER_HALF: char = '▀'; // '🮑';
        const LOWER_HALF: char = '▄'; // '🮒';
        const EMPTY: char = ' '; // '🮐';

        for top_row in (0..SCREEN_HEIGHT).step_by(2) {
            let bot_row = top_row + 1;
            for col in 0..SCREEN_WIDTH {
                let top_idx = top_row * SCREEN_WIDTH + col;
                let bot_idx = bot_row * SCREEN_WIDTH + col;
                let top_val = self.screen[top_idx];
                let bot_val = self.screen[bot_idx];

                let chr = if top_val && bot_val {
                    FULL
                } else if top_val {
                    UPPER_HALF
                } else if bot_val {
                    LOWER_HALF
                } else {
                    EMPTY
                };

                print!("{}", chr);
            }
            println!();
        }
    }
}
