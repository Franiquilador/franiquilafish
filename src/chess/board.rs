use crate::chess::piece::{Piece::*, Color::*};
use crate::chess::piece::{Piece, Color};

pub struct Board {
    fen: String,
    pieces: [[Piece; 8]; 8],
}

impl Board {
    pub fn new() -> Self {
        let initial_pos = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        Board {
            fen: initial_pos,
            pieces: [[Rook(White), Knight(White), Bishop(White), Queen(White), King(White), Bishop(White), Knight(White), Rook(White)],
            [Pawn(White), Pawn(White), Pawn(White), Pawn(White), Pawn(White), Pawn(White), Pawn(White), Pawn(White)],
            [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
            [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
            [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
            [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
            [Pawn(Black), Pawn(Black), Pawn(Black), Pawn(Black), Pawn(Black), Pawn(Black), Pawn(Black), Pawn(Black),],
            [Rook(Black), Knight(Black), Bishop(Black), Queen(Black), King(Black), Bishop(Black), Knight(Black), Rook(Black)]]
        }
    }

    pub fn getFen(&self) -> &String {
        &self.fen
    }

    pub fn setCurrentPlayerInFen(&mut self) {
        todo!()
    }

    
}