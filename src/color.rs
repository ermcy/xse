use std::ops::Not;

#[derive(Copy, Clone, Debug)]
pub enum Color {
    White = isize::MIN,
    Black = isize::MAX,
}

impl Color {
    // for FEN parsing
    pub fn from_str(value: &str) -> Self {
        match value {
            "White" | "white" | "W" | "w" => Color::White,
            "Black" | "black" | "B" | "b" => Color::Black,
            _ => unreachable!("Invalid color."),
        }
    }
}

impl Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Color::White => { Color::Black }
            Color::Black => { Color::White }
        }
    }
}
