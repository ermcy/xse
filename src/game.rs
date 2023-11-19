use crate::bitboard::Bitboard;
use crate::color::Color;
use crate::constants::{NOT_A_FILE, NOT_AB_FILE, NOT_H_FILE, NOT_HG_FILE};
use crate::pieces::Piece;
use crate::position::Position;

type PawnAttacks = [[u64; 64]; 2];
type KnightAttacks = [u64; 64];
type BishopAttacks = [u64; 64];
type RookAttacks = [u64; 64];
type QueenAttacks = [u64; 64];
type KingAttacks = [u64; 64];

#[derive(Debug)]
pub struct Game {
    bitboard: u64,
    turn: Color,
    moves: u64,
    pawn_attacks: PawnAttacks,
    knight_attacks: KnightAttacks,
    bishop_attacks: BishopAttacks,
    rook_attacks: RookAttacks,
    queen_attacks: QueenAttacks,
    king_attacks: KingAttacks,

    // position of all pieces on the board
    // there can only be 8 pawns, 2 knights, 2 bishops, 2 rooks, 1 queen, and 1 king at the start of the game
    // but after a pawn promotion, there can be more than 1 queen/rook/bishop/knight so we assume there can be 10 of each
    // but we can't have more than 10 of each piece because there are only 16 pieces per side
    white_pawns: Vec<Position>,
    black_pawns: Vec<Position>,
    white_knights: Vec<Position>,
    black_knights: Vec<Position>,
    white_bishops: Vec<Position>,
    black_bishops: Vec<Position>,
    white_rooks: Vec<Position>,
    black_rooks: Vec<Position>,
    white_queen: Vec<Position>,
    black_queen: Vec<Position>,
    white_king: Position,
    black_king: Position,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Game {
            bitboard: 0u64,
            turn: Color::White,
            moves: 0,
            pawn_attacks: [[0u64; 64]; 2],
            knight_attacks: [0u64; 64],
            bishop_attacks: [0u64; 64],
            rook_attacks: [0u64; 64],
            queen_attacks: [0u64; 64],
            king_attacks: [0u64; 64],
            white_pawns: Position::white_pawns_starting_position(),
            black_pawns: Position::black_pawns_starting_position(),
            white_knights: Position::white_knights_starting_position(),
            black_knights: Position::black_knights_starting_position(),
            white_bishops: Position::white_bishops_starting_position(),
            black_bishops: Position::black_bishops_starting_position(),
            white_rooks: Position::white_rooks_starting_position(),
            black_rooks: Position::black_rooks_starting_position(),
            white_queen: Position::white_queen_starting_position(),
            black_queen: Position::black_queen_starting_position(),
            white_king: Position::white_king_starting_position(),
            black_king: Position::black_king_starting_position(),
        };
        game.init_legal_moves_db();
        return game;
    }
    pub fn make_move(&mut self, from: u8, to: u8, piece: Piece) {
        todo!("Implement make_move");
    }

    #[inline(always)]
    pub fn print(&self) {
        let mut squares = [Piece::NULL; 64];
        // idk how to do this better
        for white_pawn in &self.white_pawns {
            let square = white_pawn.to_square_coordinates();
            squares[square] = Piece::Pawn(Color::White);
        }
        for black_pawn in &self.black_pawns {
            let square = black_pawn.to_square_coordinates();
            squares[square] = Piece::Pawn(Color::Black);
        }
        for white_knight in &self.white_knights {
            let square = white_knight.to_square_coordinates();
            squares[square] = Piece::Knight(Color::White);
        }
        for black_knight in &self.black_knights {
            let square = black_knight.to_square_coordinates();
            squares[square] = Piece::Knight(Color::Black);
        }
        for white_bishop in &self.white_bishops {
            let square = white_bishop.to_square_coordinates();
            squares[square] = Piece::Bishop(Color::White);
        }
        for black_bishop in &self.black_bishops {
            let square = black_bishop.to_square_coordinates();
            squares[square] = Piece::Bishop(Color::Black);
        }
        for white_rook in &self.white_rooks {
            let square = white_rook.to_square_coordinates();
            squares[square] = Piece::Rook(Color::White);
        }
        for black_rook in &self.black_rooks {
            let square = black_rook.to_square_coordinates();
            squares[square] = Piece::Rook(Color::Black);
        }
        for white_queen in &self.white_queen {
            let square = white_queen.to_square_coordinates();
            squares[square] = Piece::Queen(Color::White);
        }
        for black_queen in &self.black_queen {
            let square = black_queen.to_square_coordinates();
            squares[square] = Piece::Queen(Color::Black);
        }
        let white_king_position = &self.white_king.to_square_coordinates();
        squares[*white_king_position] = Piece::King(Color::White);
        let black_king_position = &self.black_king.to_square_coordinates();
        squares[*black_king_position] = Piece::King(Color::Black);
        for i in 0..squares.len() {
            if i % 8 == 0 {
                print!("{} ", 8 - i / 8);
            }
            print!("{} ", squares[i]);
            if i % 8 == 7 {
                println!();
            }
        }
    }

    fn mask_pawn_attack(color: Color, square: i32) -> u64 {
        let mut attacks = 0u64;
        let mut bitboard = Bitboard::new(Piece::Pawn(color));
        bitboard.set_bit(square);

        match color {
            Color::White => {
                if (bitboard.bits >> 7u64) & NOT_A_FILE != 0 {
                    attacks |= bitboard.bits >> 7u64;
                }
                if (bitboard.bits >> 9u64) & NOT_H_FILE != 0 {
                    attacks |= bitboard.bits >> 9u64;
                }
            }
            Color::Black => {
                if (bitboard.bits << 7u64) & NOT_H_FILE != 0 {
                    attacks |= bitboard.bits << 7u64;
                }
                if (bitboard.bits << 9u64) & NOT_A_FILE != 0 {
                    attacks |= bitboard.bits << 9u64;
                }
            }
        }

        return attacks;
    }
    fn mask_knight_attack(square: i32) -> u64 {
        let mut attacks = 0u64;
        let mut bitboard = Bitboard::new(Piece::Knight(Color::White));
        bitboard.set_bit(square);

        if (bitboard.bits >> 17u64) & NOT_H_FILE != 0 {
            attacks |= bitboard.bits >> 17u64;
        }
        if (bitboard.bits >> 15u64) & NOT_A_FILE != 0 {
            attacks |= bitboard.bits >> 15u64;
        }
        if (bitboard.bits >> 10u64) & NOT_HG_FILE != 0 {
            attacks |= bitboard.bits >> 10u64;
        }
        if (bitboard.bits >> 6u64) & NOT_AB_FILE != 0 {
            attacks |= bitboard.bits >> 6u64;
        }

        if (bitboard.bits << 17u64) & NOT_A_FILE != 0 {
            attacks |= bitboard.bits << 17u64;
        }
        if (bitboard.bits << 15u64) & NOT_H_FILE != 0 {
            attacks |= bitboard.bits << 15u64;
        }
        if (bitboard.bits << 10u64) & NOT_AB_FILE != 0 {
            attacks |= bitboard.bits << 10u64;
        }
        if (bitboard.bits << 6u64) & NOT_HG_FILE != 0 {
            attacks |= bitboard.bits << 6u64;
        }
        return attacks;
    }
    fn init_legal_moves_db(&mut self) {
        let mut i = 0;
        while i < 64 {
            let index = i as usize;
            self.pawn_attacks[0][index] = Self::mask_pawn_attack(Color::White, i);
            self.pawn_attacks[1][index] = Self::mask_pawn_attack(Color::Black, i);
            self.knight_attacks[index] = Self::mask_knight_attack(i);
            i += 1;
        }
    }
}
