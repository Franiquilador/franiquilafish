use crate::chess::move_square::{Move, Square, Promotion};
// use crate::chess::game::Color;
use crate::chess::engine::Color;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub enum Piece {
    King,
    #[default]
    Pawn, // Pawn is the default variant, but it does not matter, it is just to initialize pieces in the Zobrist table
    Knight,
    Bishop,
    Rook,
    Queen,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct ChessPiece {
    pub color: Color,
    pub piece: Piece,
}

impl ChessPiece {
    pub fn value(&self) -> i32 {
        match self.color {
            Color::White => {
                match self.piece {
                    Piece::Rook => 500,
                    Piece::Knight => 300,
                    Piece::Bishop => 300,
                    Piece::Queen => 900,
                    Piece::King => 0,
                    Piece::Pawn => 100,
                }
            },
            Color::Black => {
                match self.piece {
                    Piece::Rook => -500,
                    Piece::Knight => -300,
                    Piece::Bishop => -300,
                    Piece::Queen => -900,
                    Piece::King => 0,
                    Piece::Pawn => -100,
                }
            },
        }
    }

    pub fn all_moves(&self, pos: &Square) -> Vec<Move> { // calcula os movimentos potenciais de cada peça, a maior parte sao ilegais
        let mut moves = vec![];

        match self.piece {
            Piece::Pawn => {
                match self.color {
                    Color::Black => {
                        match pos.offset(0, -2) { //andar 2 para a frente
                            Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                            None => {}
                        };

                        match pos.offset(0, -1) { //andar 1 para a frente
                            Some(s) => {
                                moves.push(Move::from_squares(*pos, s, None));
                            },
                            None => {}
                        };
                
                        //comer na diagonal direita
                        match pos.offset(1, -1) { // file offset of one means "b" if pos is in "a"
                            Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                            None => {}
                        };

                        match pos.offset(-1, -1) { //comer na diagonal esquerda
                            Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                            None => {}
                        };
                    },

                    Color::White => {
                        match pos.offset(0, 2) { //andar 2 para a frente
                            Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                            None => {}
                        };

                        match pos.offset(0, 1) { //andar 1 para a frente
                            Some(s) => {
                                moves.push(Move::from_squares(*pos, s, None));
                            },
                            None => {}
                        };
                
                        //comer na diagonal direita
                        match pos.offset(1, 1) { // file offset of one means "b" if pos is in "a"
                            Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                            None => {}
                        };

                        match pos.offset(-1, 1) { //comer na diagonal esquerda
                            Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                            None => {}
                        };
                    },
                }
            },

            Piece::Knight => {
                match pos.offset(-1, 2) {
                    Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                    None => {}
                };

                match pos.offset(1, 2) {
                    Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                    None => {}
                };

                match pos.offset(2, 1) {
                    Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                    None => {}
                };

                match pos.offset(2, -1) {
                    Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                    None => {}
                };

                match pos.offset(1, -2) {
                    Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                    None => {}
                };

                match pos.offset(-1, -2) {
                    Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                    None => {}
                };

                match pos.offset(-2, -1) {
                    Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                    None => {}
                };

                match pos.offset(-2, 1) {
                    Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                    None => {}
                };
            }

            Piece::Bishop => {
                for i in 1..8 {
                    match pos.offset(i, i.into()) {
                        Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                        None => {}
                    };

                    match pos.offset(i, (-i).into()) {
                        Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                        None => {}
                    };

                    match pos.offset(-i, (-i).into()) {
                        Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                        None => {}
                    };

                    match pos.offset(-i, i.into()) {
                        Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                        None => {}
                    };
                }
            }

            Piece::Rook => {
                for i in 1..8 {
                    match pos.offset(i, 0) {
                        Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                        None => {}
                    };

                    match pos.offset(-i, 0) {
                        Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                        None => {}
                    };

                    match pos.offset(0, i.into()) {
                        Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                        None => {}
                    };

                    match pos.offset(0, (-i).into()) {
                        Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                        None => {}
                    };
                }
            }

            Piece::Queen => {
                for i in 1..8 {
                    //bishops moves
                    match pos.offset(i, i.into()) {
                        Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                        None => {}
                    };

                    match pos.offset(i, (-i).into()) {
                        Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                        None => {}
                    };

                    match pos.offset(-i, (-i).into()) {
                        Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                        None => {}
                    };

                    match pos.offset(-i, i.into()) {
                        Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                        None => {}
                    };

                    //rooks moves
                    match pos.offset(i, 0) {
                        Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                        None => {}
                    };

                    match pos.offset(-i, 0) {
                        Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                        None => {}
                    };

                    match pos.offset(0, i.into()) {
                        Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                        None => {}
                    };

                    match pos.offset(0, (-i).into()) {
                        Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                        None => {}
                    };
                }
            }

            Piece::King => {
                match pos.offset(0, 1) { // cima
                    Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                    None => {}
                };

                match pos.offset(1, 0) { // direita
                    Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                    None => {}
                };

                match pos.offset(1, 1) {
                    Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                    None => {}
                };

                match pos.offset(0, -1) { // baixo
                    Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                    None => {}
                };

                match pos.offset(-1, 0) { // esquerda
                    Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                    None => {}
                };

                match pos.offset(-1, -1) {
                    Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                    None => {}
                };

                match pos.offset(1, -1) {
                    Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                    None => {}
                };

                match pos.offset(-1, 1) {
                    Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                    None => {}
                };
            }
            _ => { 
                // println!("faltam calcular moves de outras peças")
            },
        };

        moves
    }

    pub fn color(&self) -> Color {
        self.color
    }
}
