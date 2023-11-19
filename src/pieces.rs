use std::fmt::Formatter;

use crate::color::Color;

#[derive(Copy, Clone, Debug)]
#[repr(isize)]
pub enum Piece {
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
    NULL = isize::MAX,
}

impl Piece {
    pub fn color(&self) -> Color {
        return match self {
            Piece::Pawn(c)
            | Piece::Knight(c)
            | Piece::Bishop(c)
            | Piece::Rook(c)
            | Piece::Queen(c)
            | Piece::King(c) => *c,
            _ => Color::White
        };
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let color = self.color();
        let piece = match color {
            Color::White => {
                match self {
                    Piece::Pawn(_) => "♙",
                    Piece::Knight(_) => "♘",
                    Piece::Bishop(_) => "♗",
                    Piece::Rook(_) => "♖",
                    Piece::Queen(_) => "♕",
                    Piece::King(_) => "♔",
                    Piece::NULL => " ",
                }
            }
            Color::Black => {
                match self {
                    Piece::Pawn(_) => "♟︎",
                    Piece::Knight(_) => "♞",
                    Piece::Bishop(_) => "♝",
                    Piece::Rook(_) => "♜",
                    Piece::Queen(_) => "♛",
                    Piece::King(_) => "♚",
                    Piece::NULL => " ",
                }
            }
        };
        write!(f, "{}", piece)
    }
}
