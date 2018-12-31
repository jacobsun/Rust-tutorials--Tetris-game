mod tetriminoes;
use crate::tetriminoes::*;
use piston_window::types::Color;
use piston_window::*;

const BLOCK_SIZE: usize = 20;
const COL: usize = 20;
const ROW: usize = 24;
const MOVE_PERIOD: f64 = 0.5;

type BoardType = [[u8; COL as usize]; ROW as usize];

fn main() {
    let mut tetris = Tetris::new();
    let mut wait_time = 0.0f64;
    let mut operation = Operation::Move;
    let colors: [Color; 7] = [
        [0.30, 0.00, 0.00, 1.0],
        [0.00, 0.30, 0.00, 1.0],
        [0.00, 0.00, 0.30, 1.0],
        [0.00, 0.00, 0.60, 1.0],
        [0.00, 0.60, 0.60, 1.0],
        [0.60, 0.00, 0.60, 1.0],
        [0.70, 0.00, 0.30, 1.0],
    ];
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    let mut window: PistonWindow = WindowSettings::new(
        "Tetris Rust",
        ((BLOCK_SIZE * COL) as f64, (BLOCK_SIZE * ROW) as f64),
    )
    .exit_on_esc(true)
    .build()
    .unwrap();
    let img = assets.join("gameover.png");
    let texture = Texture::from_path(
        &mut window.factory,
        &img,
        Flip::None,
        &TextureSettings::new(),
    )
    .unwrap();
    tetris.current_piece = Tetris::create_piece();
    while let Some(e) = window.next() {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Down => {
                    if operation != Operation::GameOver {
                        operation = Operation::Move;
                        tetris.move_piece((0, 1), &mut operation);
                    }
                }
                Key::Up => {
                    //  freezing will be delayed to makes game fun,
                    //  they also need to be canceled if the status changed
                    if operation != Operation::GameOver {
                        operation = Operation::Move;
                        tetris.rotate_piece();
                    }
                }
                Key::Left => {
                    if operation != Operation::GameOver {
                        operation = Operation::Move;
                        if !tetris.check_piece(tetris.current_piece.x - 1, tetris.current_piece.y) {
                            tetris.move_piece((-1, 0), &mut operation)
                        }
                    }
                }
                Key::Right => {
                    if operation != Operation::GameOver {
                        operation = Operation::Move;
                        if !tetris.check_piece(tetris.current_piece.x + 1, tetris.current_piece.y) {
                            tetris.move_piece((1, 0), &mut operation)
                        }
                    }
                }
                Key::Return => {
                    if operation == Operation::GameOver {
                        tetris = Tetris::new();
                        tetris.current_piece = Tetris::create_piece();
                        operation = Operation::Move;
                    }
                }
                _ => (),
            }
        }
        window.draw_2d(&e, |c, g| {
            clear([1.0, 1.0, 1.0, 1.0], g);
            tetris.draw_piece(&colors, &c, g);
            tetris.draw_board(&colors, &c, g);
            tetris.draw_shadow(&colors, &c, g);
            draw_gameover(&texture, &mut operation, &c, g);
        });
        e.update(|arg| {
            wait_time += arg.dt;
            if operation == Operation::Clear {
                wait_time = MOVE_PERIOD + 1.0f64;
            }
            if wait_time > MOVE_PERIOD {
                match operation {
                    Operation::Move => tetris.move_piece((0, 1), &mut operation),
                    Operation::Lock => {
                        tetris.lock_piece(&mut operation);
                        if operation != Operation::GameOver {
                            operation = Operation::Clear;
                        }
                    }
                    Operation::Clear => {
                        tetris.clear_line();
                        operation = Operation::Move;
                    }
                    Operation::GameOver => (),
                }
                wait_time = 0.0;
            }
        });
    }
}
fn to_coord(n: i32) -> f64 {
    f64::from(n)
}
#[derive(PartialEq, Debug)]
enum Operation {
    Move,
    Lock,
    Clear,
    GameOver,
}

struct Tetris {
    board: BoardType,
    current_piece: Tetrimino,
}

impl Tetris {
    fn new() -> Tetris {
        Tetris {
            board: [[0u8; COL]; ROW],
            current_piece: Tetris::create_piece(),
        }
    }
    fn create_piece() -> Tetrimino {
        static mut PREV: u8 = 7;
        let mut rand_nb = rand::random::<u8>() % 7;
        if unsafe { PREV } == rand_nb {
            rand_nb = rand::random::<u8>() % 7;
        }
        unsafe {
            PREV = rand_nb;
        }
        match rand_nb {
            0 => TetriminoI::new(),
            1 => TetriminoJ::new(),
            2 => TetriminoL::new(),
            3 => TetriminoO::new(),
            4 => TetriminoS::new(),
            5 => TetriminoZ::new(),
            6 => TetriminoT::new(),
            _ => unreachable!(),
        }
    }
    fn move_piece(&mut self, dir: (isize, usize), op: &mut Operation) {
        let next_x = self.current_piece.x + dir.0;
        let next_y = self.current_piece.y + dir.1;

        if self.check_piece(next_x, next_y) {
            *op = Operation::Lock;
        } else {
            self.current_piece.x = next_x;
            self.current_piece.y = next_y;
        }
    }
    fn check_piece(&self, x: isize, y: usize) -> bool {
        for (r_i, r) in self.current_piece.states[self.current_piece.current_state]
            .iter()
            .enumerate()
        {
            for (c_i, c) in r.iter().enumerate() {
                if *c != 0 {
                    if r_i + y >= ROW
                        || c_i as isize + x < 0
                        || (c_i as isize + x) as usize >= COL
                        || self.board[r_i + y][(c_i as isize + x) as usize] != 0
                    {
                        return true;
                    }
                }
            }
        }
        false
    }
    fn lock_piece(&mut self, operation: &mut Operation) {
        let piece = &mut self.current_piece;
        if piece.y == 0 {
            *operation = Operation::GameOver;
            return;
        }
        let state = piece.states[piece.current_state];
        for (r_i, r) in state.iter().enumerate() {
            for (c_i, c) in r.iter().enumerate() {
                if *c != 0 {
                    self.board[r_i + piece.y][(c_i as isize + piece.x as isize) as usize] = *c
                }
            }
        }

        self.current_piece = Tetris::create_piece();
    }
    fn check_with_state(&self, state_index: usize, x: isize, y: usize) -> bool {
        for (r_i, r) in self.current_piece.states[state_index].iter().enumerate() {
            for (c_i, c) in r.iter().enumerate() {
                if *c != 0 {
                    if r_i + y >= ROW
                        || c_i as isize + x < 0
                        || (c_i as isize + x) as usize >= COL
                        || self.board[r_i + y][(c_i as isize + x) as usize] != 0
                    {
                        return true;
                    }
                }
            }
        }
        false
    }
    fn rotate_piece(&mut self) {
        let mut next_state = self.current_piece.current_state + 1;
        if next_state >= self.current_piece.states.len() {
            next_state = 0;
        }

        let blind_try = [0isize, 1, -1, 2, -2, 3, -3];
        for n in blind_try.iter() {
            if !self.check_with_state(next_state, self.current_piece.x + n, self.current_piece.y) {
                self.current_piece.x += n;
                self.current_piece.current_state = next_state;
                break;
            }
        }
    }
    fn draw_piece(&self, colors: &[Color], con: &Context, g: &mut G2d) {
        let piece = &self.current_piece;
        let state = piece.states[piece.current_state];
        for (r_i, row) in state.iter().enumerate() {
            for (c_i, col) in row.iter().enumerate() {
                if *col != 0 {
                    rectangle(
                        colors[*col as usize - 1],
                        [
                            to_coord((piece.x as i32 + c_i as i32) * BLOCK_SIZE as i32),
                            to_coord((piece.y + r_i) as i32 * BLOCK_SIZE as i32),
                            to_coord(BLOCK_SIZE as i32),
                            to_coord(BLOCK_SIZE as i32),
                        ],
                        con.transform,
                        g,
                    );
                }
            }
        }
    }
    fn draw_board(&self, colors: &[Color], con: &Context, g: &mut G2d) {
        for (r_i, row) in self.board.iter().enumerate() {
            for (c_i, col) in row.iter().enumerate() {
                if *col != 0 {
                    rectangle(
                        colors[*col as usize - 1],
                        [
                            to_coord(c_i as i32 * BLOCK_SIZE as i32),
                            to_coord(r_i as i32 * BLOCK_SIZE as i32),
                            to_coord(BLOCK_SIZE as i32),
                            to_coord(BLOCK_SIZE as i32),
                        ],
                        con.transform,
                        g,
                    );
                }
            }
        }
    }
    fn draw_shadow(&self, colors: &[Color], con: &Context, g: &mut G2d) {
        let mut shadow_y: usize = 0;
        for i in (0..ROW).rev() {
            if !self.check_piece(self.current_piece.x, i) {
                shadow_y = i;
                break;
            }
        }

        if shadow_y < 5 || shadow_y == self.current_piece.y {
            return;
        }

        let piece = &self.current_piece;
        let state = piece.states[piece.current_state];
        for (r_i, row) in state.iter().enumerate() {
            for (c_i, col) in row.iter().enumerate() {
                if *col != 0 {
                    let mut color = colors[*col as usize - 1];
                    color[color.len() - 1] = 0.7;
                    rectangle(
                        color,
                        [
                            to_coord((piece.x as i32 + c_i as i32) * BLOCK_SIZE as i32),
                            to_coord((shadow_y + r_i) as i32 * BLOCK_SIZE as i32),
                            to_coord(BLOCK_SIZE as i32),
                            to_coord(BLOCK_SIZE as i32),
                        ],
                        con.transform,
                        g,
                    );
                }
            }
        }
    }
    fn clear_line(&mut self) {
        let mut lines = vec![];
        for (r, row) in self.board.iter().enumerate() {
            if row.iter().all(|block| *block != 0) {
                lines.push(r);
            }
        }
        if lines.is_empty() {
            return;
        }

        let mut index = *(lines.last().unwrap());
        loop {
            if index < lines.len() {
                break;
            }
            self.board[index] = self.board[index - lines.len()];
            index -= 1;
        }
    }
}

fn draw_gameover(img: &G2dTexture, op: &mut Operation, con: &Context, g: &mut G2d) {
    if *op != Operation::GameOver {
        return;
    }
    image(img, con.transform.trans(50.0, 50.0), g);
}
