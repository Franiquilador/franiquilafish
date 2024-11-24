use crate::chess::piece::{Piece as P, Color as C};
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
            pieces: [[P::Rook(C::Black), P::Knight(C::Black), P::Bishop(C::Black), P::Queen(C::Black), P::King(C::Black), P::Bishop(C::Black), P::Knight(C::Black), P::Rook(C::Black)],
            [P::Pawn(C::Black), P::Pawn(C::Black), P::Pawn(C::Black), P::Pawn(C::Black), P::Pawn(C::Black), P::Pawn(C::Black), P::Pawn(C::Black), P::Pawn(C::Black)],
            [P::Empty, P::Empty, P::Empty, P::Empty, P::Empty, P::Empty, P::Empty, P::Empty],
            [P::Empty, P::Empty, P::Empty, P::Empty, P::Empty, P::Empty, P::Empty, P::Empty],
            [P::Empty, P::Empty, P::Empty, P::Empty, P::Empty, P::Empty, P::Empty, P::Empty],
            [P::Empty, P::Empty, P::Empty, P::Empty, P::Empty, P::Empty, P::Empty, P::Empty],
            [P::Pawn(C::White), P::Pawn(C::White), P::Pawn(C::White), P::Pawn(C::White), P::Pawn(C::White), P::Pawn(C::White), P::Pawn(C::White), P::Pawn(C::White),],
            [P::Rook(C::White), P::Knight(C::White), P::Bishop(C::White), P::Queen(C::White), P::King(C::White), P::Bishop(C::White), P::Knight(C::White), P::Rook(C::White)]]
        }
    }

    pub fn get_fen(&self) -> &String {
        &self.fen
    }

    pub fn set_current_player_in_fen(&mut self, active_player: Color) {
        let mut substrings: Vec<&str> = self.fen.split(" ").collect();
        if substrings.len() > 1 {
            match active_player {
                C::White => substrings[1] = "b",
                C::Black => substrings[1] = "w",
            }
        }
    }

    pub fn get_pieces(&self) -> &[[Piece; 8]; 8] {
        &self.pieces
    }
    
}