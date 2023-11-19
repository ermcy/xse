#[derive(Copy, Clone, Debug)]
pub struct Position {
    row: usize,
    col: usize,
}

impl Position {
    pub const fn new(row: usize, col: usize) -> Self {
        return Position { row, col };
    }
    pub const fn row(&self) -> usize {
        return self.row;
    }
    pub const fn col(&self) -> usize {
        return self.col;
    }
    pub const fn to_square_coordinates(&self) -> usize {
        return self.row * 8 + self.col;
    }
    pub fn white_pawns_starting_position() -> Vec<Position> {
        let result = vec![
            Position { row: 1, col: 0 },
            Position { row: 1, col: 1 },
            Position { row: 1, col: 2 },
            Position { row: 1, col: 3 },
            Position { row: 1, col: 4 },
            Position { row: 1, col: 5 },
            Position { row: 1, col: 6 },
            Position { row: 1, col: 7 },
        ];
        return result;
    }
    pub fn black_pawns_starting_position() -> Vec<Position> {
        let result = vec![
            Position { row: 6, col: 0 },
            Position { row: 6, col: 1 },
            Position { row: 6, col: 2 },
            Position { row: 6, col: 3 },
            Position { row: 6, col: 4 },
            Position { row: 6, col: 5 },
            Position { row: 6, col: 6 },
            Position { row: 6, col: 7 },
        ];
        return result;
    }
    pub fn white_knights_starting_position() -> Vec<Position> {
        let result = vec![Position { row: 0, col: 1 }, Position { row: 0, col: 6 }];
        return result;
    }
    pub fn black_knights_starting_position() -> Vec<Position> {
        let result = vec![Position { row: 7, col: 1 }, Position { row: 7, col: 6 }];
        return result;
    }
    pub fn white_bishops_starting_position() -> Vec<Position> {
        let result = vec![Position { row: 0, col: 2 }, Position { row: 0, col: 5 }];
        return result;
    }
    pub fn black_bishops_starting_position() -> Vec<Position> {
        let result = vec![Position { row: 7, col: 2 }, Position { row: 7, col: 5 }];
        return result;
    }
    pub fn white_rooks_starting_position() -> Vec<Position> {
        let result = vec![Position { row: 0, col: 0 }, Position { row: 0, col: 7 }];
        return result;
    }
    pub fn black_rooks_starting_position() -> Vec<Position> {
        let result = vec![Position { row: 7, col: 0 }, Position { row: 7, col: 7 }];
        return result;
    }
    pub fn white_queen_starting_position() -> Vec<Position> {
        let result = vec![Position { row: 0, col: 3 }];
        return result;
    }
    pub fn black_queen_starting_position() -> Vec<Position> {
        let result = vec![Position { row: 7, col: 3 }];
        return result;
    }
    pub fn white_king_starting_position() -> Position {
        let result = Position { row: 0, col: 4 };
        return result;
    }
    pub fn black_king_starting_position() -> Position {
        let result = Position { row: 7, col: 4 };
        return result;
    }
    pub fn print_position(&self) {
        todo!("Implement print_position");
    }
}
