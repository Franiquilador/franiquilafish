use std::cmp::max;
use std::io::{stdout, Write};

// use crate::chess::game::Color;
use crate::chess::engine::Color;
use crate::chess::move_square::{Move, Square};
use crate::chess::piece::{self, ChessPiece as CP, ChessPiece, Piece};


//file = collumn
//rank = row
#[derive(Debug, Clone)]
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
                    Some(ChessPiece {
                        color: Color::White,
                        piece: Piece::Rook,
                    }),
                    Some(ChessPiece {
                        color: Color::White,
                        piece: Piece::Knight,
                    }),
                    Some(ChessPiece {
                        color: Color::White,
                        piece: Piece::Bishop,
                    }),
                    Some(ChessPiece {
                        color: Color::White,
                        piece: Piece::Queen,
                    }),
                    Some(ChessPiece {
                        color: Color::White,
                        piece: Piece::King,
                    }),
                    Some(ChessPiece {
                        color: Color::White,
                        piece: Piece::Bishop,
                    }),
                    Some(ChessPiece {
                        color: Color::White,
                        piece: Piece::Knight,
                    }),
                    Some(ChessPiece {
                        color: Color::White,
                        piece: Piece::Rook,
                    }),
                ],
                [
                    Some(ChessPiece {
                        color: Color::White,
                        piece: Piece::Pawn,
                    }),
                    Some(ChessPiece {
                        color: Color::White,
                        piece: Piece::Pawn,
                    }),
                    Some(ChessPiece {
                        color: Color::White,
                        piece: Piece::Pawn,
                    }),
                    Some(ChessPiece {
                        color: Color::White,
                        piece: Piece::Pawn,
                    }),
                    Some(ChessPiece {
                        color: Color::White,
                        piece: Piece::Pawn,
                    }),
                    Some(ChessPiece {
                        color: Color::White,
                        piece: Piece::Pawn,
                    }),
                    Some(ChessPiece {
                        color: Color::White,
                        piece: Piece::Pawn,
                    }),
                    Some(ChessPiece {
                        color: Color::White,
                        piece: Piece::Pawn,
                    }),
                ],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [
                    Some(ChessPiece {
                        color: Color::Black,
                        piece: Piece::Pawn,
                    }),
                    Some(ChessPiece {
                        color: Color::Black,
                        piece: Piece::Pawn,
                    }),
                    Some(ChessPiece {
                        color: Color::Black,
                        piece: Piece::Pawn,
                    }),
                    Some(ChessPiece {
                        color: Color::Black,
                        piece: Piece::Pawn,
                    }),
                    Some(ChessPiece {
                        color: Color::Black,
                        piece: Piece::Pawn,
                    }),
                    Some(ChessPiece {
                        color: Color::Black,
                        piece: Piece::Pawn,
                    }),
                    Some(ChessPiece {
                        color: Color::Black,
                        piece: Piece::Pawn,
                    }),
                    Some(ChessPiece {
                        color: Color::Black,
                        piece: Piece::Pawn,
                    }),
                ],
                [
                    Some(ChessPiece {
                        color: Color::Black,
                        piece: Piece::Rook,
                    }),
                    Some(ChessPiece {
                        color: Color::Black,
                        piece: Piece::Knight,
                    }),
                    Some(ChessPiece {
                        color: Color::Black,
                        piece: Piece::Bishop,
                    }),
                    Some(ChessPiece {
                        color: Color::Black,
                        piece: Piece::Queen,
                    }),
                    Some(ChessPiece {
                        color: Color::Black,
                        piece: Piece::King,
                    }),
                    Some(ChessPiece {
                        color: Color::Black,
                        piece: Piece::Bishop,
                    }),
                    Some(ChessPiece {
                        color: Color::Black,
                        piece: Piece::Knight,
                    }),
                    Some(ChessPiece {
                        color: Color::Black,
                        piece: Piece::Rook,
                    }),
                ],
            ],
        }
    }

    pub fn get_fen(&self) -> &String {
        &self.fen
    }

    pub fn set_current_player_in_fen(&mut self, active_player: &Color) {
        let mut substrings: Vec<&str> = self.fen.split(" ").collect();
        if substrings.len() > 1 {
            match active_player {
                &Color::White => substrings[1] = "b",
                &Color::Black => substrings[1] = "w",
            }
        }
    }

    pub fn get_pieces(&self) -> [[Option<ChessPiece>; 8]; 8] {
        self.pieces.clone()
    }

    pub fn get_legal_moves(&self, moving_player: &Color) -> Vec<Move> {
        let mut valid_moves = vec![];

        for (i, row) in self.pieces.iter().enumerate() {
            
            for (j, piece) in row.iter().enumerate() {
                match piece {
                    None => {
                        // println!("o quadrado inicial não tem nenhuma peça para mexer");
                    },
                    Some(piece) => {
                        if &piece.color() != moving_player {
                            continue;
                        } else {
                            let file = num_to_file(j as u8); // j is the file index (0..7) -> (a..=h)
                            let rank = (i as i32) + 1;        // i is the rank index (0..7 → 1..8)
                        
                            let pos = Square::new(file, rank).expect("should be a valid square but it is not");
                        
                            self.insert_valid_moves(piece, &pos, &mut valid_moves, moving_player); //
                        }
                    }
                }
            }
        };
        valid_moves
    }

    //inserts the valid moves for one piece
    fn insert_valid_moves(&self, piece: &ChessPiece, pos: &Square, valid_moves: &mut Vec<Move>, color: &Color) {
        let all_moves = piece.all_moves(pos); // all moves for the piece, including invalid ones   
        
        for m in all_moves {
            if self.is_move_valid(m, color) { // check if the piece move is valid within the board context
                valid_moves.push(m);
                // println!("b")
            }
        };
    }

    fn is_move_valid(&self, m: Move, color: &Color) -> bool {
        let starting_square = &m.get_starting_square();
        let final_square = &m.final_square();

        let moving_piece = self.get_piece_at_square(starting_square).expect("there should be a piece at this position in the board");

        let final_square_piece=  self.get_piece_at_square(final_square); // should be none if there is no piece at the final square

        let rank_dif = final_square.rank - starting_square.rank;
        let file_dif = (final_square.file as i8) - (starting_square.file as i8);

        match moving_piece.piece { // check for pieces in between, pieces cant hop over another piece unless its the knight
            Piece::Bishop => {
                let rank_coef = if rank_dif < 0 {
                    -1
                } else {
                    1
                };

                let file_coef = if file_dif < 0 {
                    -1
                } else {
                    1
                };

                let steps = rank_dif.abs();
                if steps != <i8 as Into<i32>>::into(file_dif).abs() {
                    return false;  // Not a diagonal move!
                }

                for i in 1..steps { // checks the positions between the bishop and the final square
                    let new_rank = starting_square.rank + (rank_coef * i);
                    let new_file = ((starting_square.file as i8) + ((file_coef as i8) * (i as i8))) as u8 as char;

                    match Square::new(new_file, new_rank) {
                        None => return false,
                        Some(s) => if self.get_piece_at_square(&s) != None {
                            return false;
                        }
                    }
                }

                return true
            },

            Piece::Rook => {
                let is_vertical = if rank_dif == 0 { false } else { true };
                let mut steps: i8 = 0;
                let mut rank_coef = 0;
                let mut file_coef = 0;

                if is_vertical {
                    rank_coef = if rank_dif > 0 { 1 } else { -1 };
                    steps = rank_dif.abs().try_into().unwrap() 
                } else {
                    file_coef = if file_dif > 0 { 1 } else { -1 };
                    steps = file_dif.abs() 
                };
                
                for i in 1..steps { // checks the positions between the rook and the final square
                    let mut new_rank = starting_square.rank as i8;
                    let mut new_file = starting_square.file;

                    if is_vertical {
                        new_rank = (starting_square.rank as i8) + (i * (rank_coef as i8)); 
                    } else {
                        new_file = ((starting_square.file as i8) + ((file_coef as i8) * (i as i8))) as u8 as char;
                    };

                    match Square::new(new_file, new_rank.into()) {
                        Some(s) => if self.get_piece_at_square(&s) != None {
                            return false;
                        }
                        None => {
                            return false;
                        }
                    }
                }
                return true;
            },
            
            Piece::Queen => {
                let is_vertical = if rank_dif == 0 { false } else { true };
                let is_horizontal = if file_dif == 0 { false } else { true };

                let is_diagonal = if is_vertical && is_horizontal { true } else { false };

                if is_diagonal {
                    let rank_coef = if rank_dif < 0 {
                        -1
                    } else {
                        1
                    };

                    let file_coef = if file_dif < 0 {
                        -1
                    } else {
                        1
                    };

                    let steps = rank_dif.abs();
                    if steps != <i8 as Into<i32>>::into(file_dif).abs() {
                        return false;  // Not a diagonal move!
                    }

                    for i in 1..steps { // checks the positions between the bishop and the final square
                        let new_rank = starting_square.rank + (rank_coef * i);
                        let new_file = ((starting_square.file as i8) + ((file_coef as i8) * (i as i8))) as u8 as char;

                        match Square::new(new_file, new_rank) {
                            None => return false,
                            Some(s) => if self.get_piece_at_square(&s) != None {
                                return false;
                            }
                        }
                    }

                } else {
                    let mut steps: i8 = 0;
                    let mut rank_coef = 0;
                    let mut file_coef = 0;

                    if is_vertical {
                        rank_coef = if rank_dif > 0 { 1 } else { -1 };
                        steps = rank_dif.abs().try_into().unwrap() 
                    } else {
                        file_coef = if file_dif > 0 { 1 } else { -1 };
                        steps = file_dif.abs() 
                    };
                
                    for i in 1..steps { // checks the positions between the rook and the final square
                        let mut new_rank = starting_square.rank as i8;
                        let mut new_file = starting_square.file;

                        if is_vertical {
                            new_rank = (starting_square.rank as i8) + (i * (rank_coef as i8)); 
                        } else {
                            new_file = ((starting_square.file as i8) + ((file_coef as i8) * (i as i8))) as u8 as char;
                        };

                        match Square::new(new_file, new_rank.into()) {
                            Some(s) => if self.get_piece_at_square(&s) != None {
                                return false;
                            }
                            None => {
                                return false;
                            }
                        }
                    }
                    
                }
                return true;
            }

            _ => {}
        };

        match final_square_piece { // todo! this logic is not finished
            None => {// nao há peça no quadrado final
                match moving_piece.piece {
                    Piece::Pawn => {
                        if starting_square.file != final_square.file {// nao pode andar na diagonal, apenas capturar
                            false
                        } else {
                            match moving_piece.color {
                                Color::Black => {
                                    if ((starting_square.rank - final_square.rank).abs() >= 2)
                                        && (starting_square.rank != 7) {
                                        false
                                    } else {
                                        if (/*starting_square.rank == 7 &&*/ self.get_piece_at_square(&Square { rank: starting_square.rank - 1, file: starting_square.file }) != None) {
                                            false
                                        } else {
                                            true    
                                        }
                                    }
                                },
                                Color::White => {
                                    if ((starting_square.rank - final_square.rank).abs() >= 2)
                                        && (starting_square.rank != 2) {
                                        false
                                    } else {
                                        if (/*starting_square.rank == 2 &&*/ self.get_piece_at_square(&Square { rank: starting_square.rank + 1, file: starting_square.file }) != None) {
                                            false
                                        } else {
                                            true
                                        }
                                    }
                                },
                            }
                        }
                    }

                    Piece::Knight => {
                        // todo!("falta logica de pins cheques, e peças no caminho quando nao é o cavalo a mexer");
                        true
                    },
                    _ => {
                        // println!("falta a logica de peças mexerem-se com outras no caminho");
                        true
                    }
                }
            },

            Some(piece) => { // ha uma peça no quadrado final
                let piece_color = piece.color;
                if piece_color == *color { // essa peça é da mesma equipa
                    false
                } else { // é capturável/da outra equipa
                    match piece.piece {
                        Piece::King => {
                            // println!("falta a logica de cheque do rei");
                            false
                        },
                        _ => match moving_piece.piece {
                                Piece::Pawn => {
                                    if starting_square.file != final_square.file {
                                        true
                                    } else {
                                        false
                                    }
                                },

                                Piece::Knight => {
                                    // todo!("falta logica de pins cheques, e peças no caminho quando nao é o cavalo a mexer");
                                    true
                                },

                                _ => {
                                    // println!("falta a logica de peças capturarem-se");
                                    false
                                }
                            }
                    }
                }
            },
        }
    }

    pub fn update_square(&mut self, piece: Option<ChessPiece>, square: &Square) {
        let col = file_to_num(square.file);
        let row = square.rank as usize - 1;

        self.pieces[row][col as usize] = piece;
    }

    pub fn get_piece_at_square(&self, square: &Square) -> Option<ChessPiece> {
        let col = file_to_num(square.file);

        let row = square.rank as usize - 1;

        self.pieces[row][col as usize]
    }
}

fn file_to_num(c: char) -> u8 {
    if ('a'..='h').contains(&c) {
        c as u8 - b'a'          // 0..=7
    } else {
        panic!("invalid file");
    }
}

fn num_to_file(n: u8) -> char {
    if n < 8 {
        (b'a' + n) as char     // 0..=7
    } else {
        panic!("invalid file index");
    }
}
