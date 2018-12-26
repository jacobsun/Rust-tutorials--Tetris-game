use piston_window::types::Color;
use piston_window::*;
const BLOCK_SIZE: usize = 20;
const COL: usize = 20;
const ROW: usize = 24;
const UPDATE_PERIOD: f64 = 0.1;
fn main() {
    let mut wait_time = 0.0f64;
    let block_color = [0.7, 0.0, 0.0, 1.0];
    let mut block = Block {
        x: 0,
        y: 0,
        width: 1,
        height: 1,
    };
    let mut window: PistonWindow = WindowSettings::new(
        "Tetris Rust",
        ((BLOCK_SIZE * COL) as f64, (BLOCK_SIZE * ROW) as f64),
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g| {
            clear([1.0, 1.0, 1.0, 1.0], g);
            block.draw(block_color, &c, g);
        });

        event.update(|arg| {
            wait_time += arg.dt;
            if wait_time < UPDATE_PERIOD {
                return;
            }
            block.x = rand::random::<usize>() % COL;
            block.y = rand::random::<usize>() % ROW;
            wait_time = 0.0;
        });
    }
}

fn coord(n: usize) -> f64 {
    (n * BLOCK_SIZE) as f64
}

struct Block {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Block {
    fn draw(&self, color: Color, c: &Context, g: &mut G2d) {
        rectangle(
            color,
            [
                coord(self.x),
                coord(self.y),
                coord(self.width),
                coord(self.height),
            ],
            c.transform,
            g,
        );
    }
}
