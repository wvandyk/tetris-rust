use core::fmt;
use rand::Rng;
use std::collections::HashMap;

use crate::GameBoard;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TetrisPieceType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
    None,
}

#[derive(Debug, PartialEq)]
pub enum TetrisPieceState {
    Active,
    Placing,
    Placed,
}

#[derive(Debug)]
pub struct TetrisPiece {
    pub piece_type: TetrisPieceType,
    pub grid: Vec<i32>, // 2D vector to represent the shape in its current rotation
    pub x: i32,         // x position of the piece
    pub y: i32,         // y position of the piece
    pub rotation: u8,   // rotation of the piece (0-3)
    pub kick_table: HashMap<u8, Vec<(i32, i32)>>,
    pub state: TetrisPieceState,
}

impl TetrisPiece {
    pub fn new(piece_type: TetrisPieceType) -> TetrisPiece {
        let grid = match piece_type {
            TetrisPieceType::I => vec![
                0, 0, 0, 0, 0, 
                0, 0, 0, 0, 0, 
                0, 1, 1, 1, 1, 
                0, 0, 0, 0, 0, 
                0, 0, 0, 0, 0,
            ],
            TetrisPieceType::O => vec![
                0, 0, 0, 0, 0, 
                0, 0, 2, 2, 0, 
                0, 0, 2, 2, 0, 
                0, 0, 0, 0, 0, 
                0, 0, 0, 0, 0,
            ],
            TetrisPieceType::T => vec![
                0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            TetrisPieceType::S => vec![
                0, 0, 0, 0, 0, 0, 0, 4, 4, 0, 0, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            TetrisPieceType::Z => vec![
                0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            TetrisPieceType::J => vec![
                0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 6, 6, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            TetrisPieceType::L => vec![
                0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 7, 7, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            TetrisPieceType::None => vec![0; 25],
        };
        let kick_table = match piece_type {
            TetrisPieceType::T
            | TetrisPieceType::S
            | TetrisPieceType::Z
            | TetrisPieceType::J
            | TetrisPieceType::L => {
                let mut kick_table: HashMap<u8, Vec<(i32, i32)>> = HashMap::new();
                kick_table.insert(0, vec![(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)]);
                kick_table.insert(1, vec![(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)]);
                kick_table.insert(2, vec![(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)]);
                kick_table.insert(3, vec![(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)]);
                kick_table
            }
            TetrisPieceType::O => {
                let mut kick_table: HashMap<u8, Vec<(i32, i32)>> = HashMap::new();
                kick_table.insert(0, vec![(0, 0)]);
                kick_table.insert(1, vec![(0, -1)]);
                kick_table.insert(2, vec![(-1, -1)]);
                kick_table.insert(3, vec![(-1, 0)]);
                kick_table
            }
            TetrisPieceType::I => {
                let mut kick_table: HashMap<u8, Vec<(i32, i32)>> = HashMap::new();
                kick_table.insert(0, vec![(0, 0), (-1, 0), (2, 0), (-1, 0), (2, 0)]);
                kick_table.insert(1, vec![(-1, 0), (0, 0), (0, 0), (0, 1), (0, -2)]);
                kick_table.insert(2, vec![(-1, 1), (1, 1), (-2, 1), (1, 0), (-2, 0)]);
                kick_table.insert(3, vec![(0, 1), (0, 1), (0, 1), (0, -1), (0, 2)]);
                kick_table
            }
            TetrisPieceType::None => HashMap::new(),
        };
        TetrisPiece {
            piece_type,
            grid,
            x: 2,
            y: 0,
            rotation: 0,
            kick_table,
            state: TetrisPieceState::Active,
        }
    }

    fn gen_kick_vectors(&self, prev_rotation: u8, next_rotation: u8) -> Vec<(i32, i32)> {
        let mut kick_vectors: Vec<(i32, i32)> = Vec::new();
        let prev_kick_vectors: Vec<(i32, i32)> = self.kick_table[&prev_rotation].clone();
        let next_kick_vectors: Vec<(i32, i32)> = self.kick_table[&next_rotation].clone();

        prev_kick_vectors
            .iter()
            .zip(next_kick_vectors.iter())
            .for_each(|(prev, next)| {
                kick_vectors.push((prev.0 - next.0, prev.1 - next.1));
            });
        kick_vectors
    }

    pub fn new_random_piece() -> TetrisPiece {
        let mut rng = rand::thread_rng();
        let piece_type: TetrisPieceType = match rng.gen_range(0..=6) {
            0 => TetrisPieceType::I,
            1 => TetrisPieceType::O,
            2 => TetrisPieceType::T,
            3 => TetrisPieceType::S,
            4 => TetrisPieceType::Z,
            5 => TetrisPieceType::J,
            6 => TetrisPieceType::L,
            _ => panic!("Invalid piece type"),
        };
        TetrisPiece::new(piece_type)
    }

    pub fn translate(&mut self, game_board: &GameBoard, dx: i32, dy: i32) {
        if self.state == TetrisPieceState::Placed {
            return;
        }
        self.x = self.x + dx;
        self.y = self.y + dy;
        if game_board.test_placement(self) {
            return;
        } else {
            self.x = self.x - dx;
            self.y = self.y - dy;
        }
    }

    pub fn srs_rotate(&mut self, game_board: &GameBoard, cw_rotation: bool) {
        if self.state == TetrisPieceState::Placed {
            return;
        }
        let prev_rotation = self.rotation;
        let prev_x = self.x;
        let prev_y = self.y;

        if cw_rotation {
            self.rotate_cw();
        } else {
            self.rotate_ccw();
        }
        let kick_vectors = self.gen_kick_vectors(prev_rotation, self.rotation);
        for kick_vector in kick_vectors {
            self.x = self.x + kick_vector.0;
            self.y = self.y - kick_vector.1;
            if game_board.test_placement(self) {
                return;
            }
            self.x = prev_x;
            self.y = prev_y;
        }

        if cw_rotation {
            self.rotate_ccw();
        } else {
            self.rotate_cw();
        }
    }

    pub fn rotate_ccw(&mut self) {
        self.rotation = (self.rotation + 3) % 4;
        let mut tmp_grid: Vec<i32> = vec![0; self.grid.len()];

        for x in 0..5 {
            for y in 0..5 {
                tmp_grid[(x + y * 5) as usize] = self.grid[(4 - y + x * 5) as usize];
            }
        }
        self.grid = tmp_grid;
    }

    pub fn rotate_cw(&mut self) {
        self.rotation = (self.rotation + 1) % 4;
        let mut tmp_grid: Vec<i32> = vec![0; self.grid.len()];

        for x in 0..5 {
            for y in 0..5 {
                tmp_grid[(4 - y + x * 5) as usize] = self.grid[(x + y * 5) as usize];
            }
        }
        self.grid = tmp_grid;
    }
}

impl fmt::Display for TetrisPiece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.grid.iter().enumerate().for_each(|(i, x)| {
            if i % 5_usize == 0 {
                writeln!(f).unwrap();
            }
            write!(f, "{}", x).unwrap();
        });
        writeln!(f, "\n{}", self.rotation).unwrap();
        fmt::Result::Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_creation() {
        let t: TetrisPiece = TetrisPiece::new(TetrisPieceType::T);
        assert_eq!(t.piece_type, TetrisPieceType::T);
        assert_eq!(
            t.grid,
            vec![0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,]
        );
    }

    #[test]
    fn test_srs_rotation() {
        let mut b = GameBoard::new(10, 20);
        let mut t = TetrisPiece::new(TetrisPieceType::J);
        t.x = 2;
        t.y = 13;

        b.grid = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0,
            0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1,
        ];
        t.srs_rotate(&b, false);
        b.place_piece(&t);
        b.grid.iter().enumerate().for_each(|(i, x)| {
            if i % 10 == 0 {
                println!();
            }
            print!("{}", x);
        });
        println!("\n\n\n");
        assert_eq!(
            b.grid,
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 6, 0, 1,
                1, 1, 1, 1, 0, 0, 0, 6, 1, 1, 1, 1, 1, 1, 1, 1, 6, 6, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
                1, 1, 1, 1,
            ]
        );
    }
}
