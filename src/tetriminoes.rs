type StateType = [PieceType; 4];
type PieceType = [[u8; 4]; 4];
pub struct Tetrimino {
    pub states: StateType,
    pub x: isize,
    pub y: usize,
    pub current_state: usize,
}

pub trait TetriminoGenerator {
    fn new() -> Tetrimino;
}

pub struct TetriminoI;
impl TetriminoGenerator for TetriminoI {
    fn new() -> Tetrimino {
        Tetrimino {
            states: [
                [[1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                [[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0]],
                [[0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]],
                [[0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0]],
            ],
            x: 8,
            y: 0,
            current_state: 0,
        }
    }
}

pub struct TetriminoJ;
impl TetriminoGenerator for TetriminoJ {
    fn new() -> Tetrimino {
        Tetrimino {
            states: [
                [[2, 2, 2, 0], [2, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                [[2, 2, 0, 0], [0, 2, 0, 0], [0, 2, 0, 0], [0, 0, 0, 0]],
                [[0, 0, 2, 0], [2, 2, 2, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                [[2, 0, 0, 0], [2, 0, 0, 0], [2, 2, 0, 0], [0, 0, 0, 0]],
            ],
            x: 8,
            y: 0,
            current_state: 0,
        }
    }
}
pub struct TetriminoL;
impl TetriminoGenerator for TetriminoL {
    fn new() -> Tetrimino {
        Tetrimino {
            states: [
                [[3, 3, 3, 0], [0, 0, 3, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                [[0, 3, 0, 0], [0, 3, 0, 0], [3, 3, 0, 0], [0, 0, 0, 0]],
                [[3, 0, 0, 0], [3, 3, 3, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                [[3, 3, 0, 0], [3, 0, 0, 0], [3, 0, 0, 0], [0, 0, 0, 0]],
            ],
            x: 8,
            y: 0,
            current_state: 0,
        }
    }
}
pub struct TetriminoO;
impl TetriminoGenerator for TetriminoO {
    fn new() -> Tetrimino {
        Tetrimino {
            states: [
                [[4, 4, 0, 0], [4, 4, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                [[4, 4, 0, 0], [4, 4, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                [[4, 4, 0, 0], [4, 4, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                [[4, 4, 0, 0], [4, 4, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
            ],
            x: 8,
            y: 0,
            current_state: 0,
        }
    }
}
pub struct TetriminoS;
impl TetriminoGenerator for TetriminoS {
    fn new() -> Tetrimino {
        Tetrimino {
            states: [
                [[0, 5, 5, 0], [5, 5, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                [[0, 5, 0, 0], [0, 5, 5, 0], [0, 0, 5, 0], [0, 0, 0, 0]],
                [[0, 5, 5, 0], [5, 5, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                [[0, 5, 0, 0], [0, 5, 5, 0], [0, 0, 5, 0], [0, 0, 0, 0]],
            ],
            x: 8,
            y: 0,
            current_state: 0,
        }
    }
}
pub struct TetriminoZ;
impl TetriminoGenerator for TetriminoZ {
    fn new() -> Tetrimino {
        Tetrimino {
            states: [
                [[6, 6, 0, 0], [0, 6, 6, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                [[0, 0, 6, 0], [0, 6, 6, 0], [0, 6, 0, 0], [0, 0, 0, 0]],
                [[6, 6, 0, 0], [0, 6, 6, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                [[0, 0, 6, 0], [0, 6, 6, 0], [0, 6, 0, 0], [0, 0, 0, 0]],
            ],
            x: 8,
            y: 0,
            current_state: 0,
        }
    }
}
pub struct TetriminoT;
impl TetriminoGenerator for TetriminoT {
    fn new() -> Tetrimino {
        Tetrimino {
            states: [
                [[7, 7, 7, 0], [0, 7, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                [[0, 7, 0, 0], [7, 7, 0, 0], [0, 7, 0, 0], [0, 0, 0, 0]],
                [[0, 7, 0, 0], [7, 7, 7, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                [[0, 7, 0, 0], [0, 7, 7, 0], [0, 7, 0, 0], [0, 0, 0, 0]],
            ],
            x: 8,
            y: 0,
            current_state: 0,
        }
    }
}
