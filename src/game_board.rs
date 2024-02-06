use crate::TetrisPiece;

#[derive(Debug)]
pub struct GameBoard {
    pub grid: Vec<i32>,
    pub width: i32,
    pub height: i32,
}

impl GameBoard {
    pub fn new(width: i32, height: i32) -> GameBoard {
        GameBoard {
            grid: vec![0; (width * height) as usize],
            width,
            height,
        }
    }

    pub fn test_placement(&self, piece: &TetrisPiece) -> bool {
        for x in 0..5 {
            for y in 0..5 {
                if piece.grid[x + y * 5] != 0 {
                    if (piece.x + x as i32) >= self.width || (piece.y + y as i32) >= self.height {
                        return false;
                    }
                    if (piece.x + x as i32) < 0 || (piece.y + y as i32) < 0 {
                        return false;
                    }
                    if self.grid
                        [((piece.x + x as i32) + (piece.y + y as i32) * self.width) as usize]
                        != 0
                    {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn place_piece(&mut self, piece: &TetrisPiece) -> &mut GameBoard {
        for x in 0..5 {
            for y in 0..5 {
                if piece.grid[x + y * 5] != 0 {
                    self.grid
                        [((piece.x + x as i32) + (piece.y + y as i32) * self.width) as usize] =
                        piece.grid[x + y * 5];
                }
            }
        }
        self
    }

    pub fn full_lines(&self) -> Vec<i32> {
        let mut full_lines: Vec<i32> = Vec::new();
        for y in 0..self.height {
            let mut full_line: bool = true;
            for x in 0..self.width {
                if self.grid[(x + y * self.width) as usize] == 0 {
                    full_line = false;
                    break;
                }
            }
            if full_line {
                full_lines.push(y);
            }
        }
        full_lines
    }
}
