use crate::pieces::Piece;

#[derive(Debug)]
pub struct Bitboard {
    pub bits: u64,
    piece_type: Piece,
}

impl Bitboard {
    pub const fn new(piece: Piece) -> Self {
        return Bitboard {
            bits: 0u64,
            piece_type: piece,
        };
    }
    pub const fn get_piece_type(&self) -> Piece {
        return self.piece_type;
    }

    pub const fn get_bit(&self, square: i32) -> u64 {
        let shift = 1u64 << square;
        return if self.bits & shift == 0 { 0 } else { 1 };
    }
    pub fn set_bit(&mut self, square: i32) {
        let shift = 1u64 << square;
        self.bits |= shift;
    }
    pub fn clear_bit(&mut self, square: i32) {
        let shift = 1u64 << square;
        self.bits &= !shift;
    }
    pub fn print_yourself_now(&self) {
        let mut rank = 0;
        let mut file = 0;
        while rank < 8 {
            while file < 8 {
                let square_i32 = rank * 8 + file;
                if file == 0 {
                    print!("{0} ", 8 - rank);
                }
                let bit = self.get_bit(square_i32);
                // let square = Square::from_i32(square_i32);
                print!(" {0}", bit);
                file += 1;
            }
            println!();
            rank += 1;
            file = 0;
        }
        println!("   A B C D E F G H");
        println!("Bitboard: {0}", self.bits);
    }
}
