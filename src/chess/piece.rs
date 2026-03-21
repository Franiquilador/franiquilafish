use std::usize;

use crate::chess::move_square::{Move, Square};
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

const KNIGHT_TABLE: [[i32; 8]; 8] = [
    [-15, -10, -5, -5, -5, -5, -10, -15],
    [-10, 0, 5, 5, 5, 5, 0, -10],
    [-5, 5, 10, 10, 10, 10, 5, -5],
    [-5, 5, 10, 15, 15, 10, 5, -5],
    [-5, 5, 10, 15, 15, 10, 5, -5],
    [-5, 5, 10, 10, 10, 10, 5, -5],
    [-10, 0, 5, 5, 5, 5, 0, -10],
    [-15, -10, -5, -5, -5, -5, -10, -15],
];

const WHITE_KING_TABLE: [[i32; 8]; 8] = [
    [5, 10, 0, -10, -10, 0, 10, 5],
    [5, 4, -10, -10, -10, -10, 4, 5],
    [-8, -13, -13, -13, -13, -13, -13, -8],
    [-8, -14, -14, -15, -15, -14, -14, -8],
    [-9, -17, -17, -20, -20, -17, -17, -9],
    [-8, -17, -17, -20, -20, -17, -17, -8],
    [-10, -17, -18, -20, -20, -18, -17, -10],
    [-10, -17, -18, -20, -20, -18, -17, -10],
];

const BLACK_KING_TABLE: [[i32; 8]; 8] = [
    [10, 17, 18, 20, 20, 18, 17, 10],
    [10, 17, 18, 20, 20, 18, 17, 10],
    [8, 17, 17, 20, 20, 17, 17, 8],
    [9, 17, 17, 20, 20, 17, 17, 9],
    [8, 14, 14, 15, 15, 14, 14, 8],
    [8, 13, 13, 13, 13, 13, 13, 8],
    [-5, -4, 10, 10, 10, 10, -4, -5],
    [-5, -10, 0, 10, 10, 0, -10, -5],
];

impl ChessPiece {
    pub fn value(&self, row: usize, col: usize) -> i32 {
        let mut value: i32 = 0;

        match self.piece {
            Piece::Rook => value += 500,
            Piece::Knight => value += 300,
            Piece::Bishop => value += 300,
            Piece::Queen => value += 900,
            Piece::King => value += 0,
            Piece::Pawn => value += 100,
        };

        if self.color == Color::Black {
            value = -value;
        };

        value += self.piece_square_table(row, col);

        value
    }

    fn piece_square_table(&self, row: usize, col: usize) -> i32 {
        match self.piece {
            Piece::King => match self.color {
                Color::Black => BLACK_KING_TABLE[row][col],
                Color::White => WHITE_KING_TABLE[row][col],
            },

            Piece::Knight => {
                if self.color == Color::Black {
                    -KNIGHT_TABLE[row][col]
                } else {
                    KNIGHT_TABLE[row][col]
                }
            }

            _ => 0,
        }
    }
/* everything bellow inside /// will be rendered as markdown,
and also rendered as rust code and doc test there are 3 backticks forming a block:
```
code here
```
*/

    /// `if let` in rust means "if I can let this patern match, then do this".
    /// 
    /// The `let` keyword in rust is used exclusively for patterns
    /// when assigning a variable, the let keyword is always for a pattern on the left, and a value on the right:
    /// 
    /// the `match` bellow is equivalent to the `if let`, just more verbose:
    ///    
    /// comer na diagonal direita
    /// 
    /// ```
    /// match pos.offset(1, -1) {
    ///     Some(s) => moves.push(Move::from_squares(*pos, s, None)),
    ///     None => {}
    /// };
    /// 
    /// if let Some(s) = pos.offset(1, -1) {
    ///     moves.push(Move::from_squares(*pos, s, None));
    /// }
    /// ```
    /// 
    pub fn all_moves(&self, pos: &Square) -> Vec<Move> {
        // calcula os movimentos potenciais de cada peça, a maior parte sao ilegais
        let mut moves = vec![];

        match self.piece {
            Piece::Pawn => {
                match self.color {
                    Color::Black => {
                        match pos.offset(0, -2) {
                            //andar 2 para a frente
                            Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                            None => {}
                        };

                        match pos.offset(0, -1) {
                            //andar 1 para a frente
                            Some(s) => {
                                moves.push(Move::from_squares(*pos, s, None));
                            }
                            None => {}
                        };

                        //comer na diagonal direita
                        match pos.offset(1, -1) {
                            // file offset of one means "b" if pos is in "a"
                            Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                            None => {}
                        };

                        match pos.offset(-1, -1) {
                            //comer na diagonal esquerda
                            Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                            None => {}
                        };
                    }

                    Color::White => {
                        match pos.offset(0, 2) {
                            //andar 2 para a frente
                            Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                            None => {}
                        };

                        match pos.offset(0, 1) {
                            //andar 1 para a frente
                            Some(s) => {
                                moves.push(Move::from_squares(*pos, s, None));
                            }
                            None => {}
                        };

                        //comer na diagonal direita
                        match pos.offset(1, 1) {
                            // file offset of one means "b" if pos is in "a"
                            Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                            None => {}
                        };

                        match pos.offset(-1, 1) {
                            //comer na diagonal esquerda
                            Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                            None => {}
                        };
                    }
                }
            }

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
                match pos.offset(0, 1) {
                    // cima
                    Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                    None => {}
                };

                match pos.offset(1, 0) {
                    // direita
                    Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                    None => {}
                };

                match pos.offset(1, 1) {
                    Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                    None => {}
                };

                match pos.offset(0, -1) {
                    // baixo
                    Some(s) => moves.push(Move::from_squares(*pos, s, None)),
                    None => {}
                };

                match pos.offset(-1, 0) {
                    // esquerda
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
        };

        moves
    }

    pub fn color(&self) -> Color {
        self.color
    }
}
