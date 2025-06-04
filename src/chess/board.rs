use crate::chess::game::Color;
use crate::chess::move_square::{Move, Square};
use crate::chess::piece::{ChessPiece as CP, ChessPiece, Piece as P};

pub struct Board {
    fen: String,
    pieces: [[Option<ChessPiece>; 8]; 8],
}

const INITIAL_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

impl Board {
    pub fn new() -> Self {
        let initial_pos = String::from(INITIAL_FEN);
        Board {
            fen: initial_pos,
            pieces: [
                [
                    Some(CP::Black(P::Rook)),
                    Some(CP::Black(P::Knight)),
                    Some(CP::Black(P::Bishop)),
                    Some(CP::Black(P::Queen)),
                    Some(CP::Black(P::King)),
                    Some(CP::Black(P::Bishop)),
                    Some(CP::Black(P::Knight)),
                    Some(CP::Black(P::Rook)),
                ],
                [
                    Some(CP::Black(P::Pawn)),
                    Some(CP::Black(P::Pawn)),
                    Some(CP::Black(P::Pawn)),
                    Some(CP::Black(P::Pawn)),
                    Some(CP::Black(P::Pawn)),
                    Some(CP::Black(P::Pawn)),
                    Some(CP::Black(P::Pawn)),
                    Some(CP::Black(P::Pawn)),
                ],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [
                    Some(CP::White(P::Pawn)),
                    Some(CP::White(P::Pawn)),
                    Some(CP::White(P::Pawn)),
                    Some(CP::White(P::Pawn)),
                    Some(CP::White(P::Pawn)),
                    Some(CP::White(P::Pawn)),
                    Some(CP::White(P::Pawn)),
                    Some(CP::White(P::Pawn)),
                ],
                [
                    Some(CP::White(P::Rook)),
                    Some(CP::White(P::Knight)),
                    Some(CP::White(P::Bishop)),
                    Some(CP::White(P::Queen)),
                    Some(CP::White(P::King)),
                    Some(CP::White(P::Bishop)),
                    Some(CP::White(P::Knight)),
                    Some(CP::White(P::Rook)),
                ],
            ],
        }
    }

    pub fn get_fen(&self) -> &String {
        &self.fen
    }

    pub fn set_current_player_in_fen(&mut self, active_player: Color) {
        let mut substrings: Vec<&str> = self.fen.split(" ").collect();
        if substrings.len() > 1 {
            match active_player {
                Color::White => substrings[1] = "b",
                Color::Black => substrings[1] = "w",
            }
        }
    }

    pub fn get_pieces(&self) -> &[[Option<ChessPiece>; 8]; 8] {
        &self.pieces
    }

    pub fn is_move_valid(&self, m: Move) -> bool {
        false
    }
}
