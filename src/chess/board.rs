use std::cmp::max;
use std::{convert, vec};
use std::io::{stdout, Write};

// use crate::chess::game::Color;
use crate::chess::engine::Color;
use crate::chess::move_square::{Move, Square, Promotion};
use crate::chess::piece::{self, ChessPiece as CP, ChessPiece, Piece};


//file = collumn
//rank = row
#[derive(Debug, Clone)]
pub struct Board {
    fen: String,
    pub current_player: Color,
    en_passant: Option<Square>, // target square for en passant if it exists
    castling_rights: Vec<char>,
    // pub half_moves: i32, // increments on every turn~, used to skip already aplied moves in UCI comunication
    half_move_clock: i32, // registered on the fen, use for the 50 move no capture etc draw rule
    full_moves: i32, //
    pieces: [[Option<ChessPiece>; 8]; 8],
}

#[derive(Debug, Clone)]
enum CastlingSide {
    All,
    King,
    Queen,
}

#[derive(Debug, Clone)]
enum Castling {
    White(Option<CastlingSide>),
    Black(Option<CastlingSide>),
}

const INITIAL_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"; // capital letters are white

impl Board {
    pub fn new() -> Self { // from starting position
        let initial_pos = String::from(INITIAL_FEN);
        Board {
            fen: initial_pos,
            // castling_rights: [Castling::White(Some(CastlingSide::All)), Castling::Black(Some(CastlingSide::All))],
            castling_rights: vec!['K', 'Q', 'k', 'q'],
            en_passant: None,
            // half_moves: 0,
            half_move_clock: 0,
            full_moves: 1,
            current_player: Color::White,
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

    pub fn from_fen(fen_parts: Vec<&str>) -> Self {
        let n_full_moves = fen_parts.get(5).expect("fen should have full move").parse().unwrap();
        let current_player = Self::current_player_from_fen(fen_parts.get(1)
        .expect("fen should have current player"));

        let ep_square = Self::en_passant_from_fen(fen_parts.clone());

        let mut board = Board {
            fen: fen_parts.join(" ").trim().to_string(), // joins the fen into a single string with whitespace in between
            current_player: current_player,
            castling_rights: Self::castling(fen_parts.get(2).expect("no castling info")),
            en_passant: None,
            half_move_clock: fen_parts.get(4).expect("fen should have halfmove clock").parse().unwrap(),
            full_moves: n_full_moves,
            // half_moves: Self::half_moves_from_fen(n_full_moves, current_player),
            pieces: Self::pieces_from_fen(fen_parts) };

        board.en_passant(ep_square);
        
        board
    }

    fn castling(rights: &str) -> Vec<char> {
        let mut vec_rights = vec![];
        
        for c in rights.chars() {
            vec_rights.push(c);
        }

        vec_rights
    }

    fn half_moves_from_fen(full_moves: i32, current_player: Color) -> i32 {
        let half_moves = ((full_moves - 1) * 2) + (if current_player == Color::Black { 1 } else { 0 });
        half_moves
    }

    pub fn update_move_counts(&mut self, player_who_just_played: Color, is_capture_or_pawn_move: bool) {
        // self.half_moves += 1;     
        if player_who_just_played == Color::Black {
            self.full_moves += 1;
        }

        if is_capture_or_pawn_move { // reset
            self.half_move_clock = 0;
        } else {
            self.half_move_clock += 1;
        }
    }

    fn current_player_from_fen(color: &str) -> Color {
        match color {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("invalid color"),
        }
    }

    fn pieces_from_fen(fen_parts: Vec<&str>) -> [[Option<ChessPiece>; 8]; 8] {
        let ranks: Vec<&str> = fen_parts.get(0)
        .expect("should have the board position in the fen")
        .split('/').rev().collect(); // '/' is more eficient than "/" in this case rev()??????

        // dbg!(ranks.clone());
        if ranks.len() != 8 {
            panic!("board should only have 8 elements");
        }

        let mut pieces_matrix: Vec<Vec<Option<ChessPiece>>> = vec![];

        for rank in ranks {
            let mut pieces_v: Vec<Option<ChessPiece>> = vec![];
            for c in rank.chars() {
                match c.to_digit(10) {
                    None => {
                        let mut piece: Option<ChessPiece> = None;
                        match c {
                            'r' => {
                                piece = Some(ChessPiece {
                                    piece: Piece::Rook,
                                    color: Color::Black,
                                })
                            },
                            'n' => {
                                piece = Some(ChessPiece {
                                    piece: Piece::Knight,
                                    color: Color::Black,
                                })
                            },
                            'b' => {
                                piece = Some(ChessPiece {
                                    piece: Piece::Bishop,
                                    color: Color::Black,
                                })
                            },
                            'q' => {
                                piece = Some(ChessPiece {
                                    piece: Piece::Queen,
                                    color: Color::Black,
                                })
                            },
                            'k' => {
                                piece = Some(ChessPiece {
                                    piece: Piece::King,
                                    color: Color::Black,
                                })
                            },
                            'p' => {
                                piece = Some(ChessPiece {
                                    piece: Piece::Pawn,
                                    color: Color::Black,
                                })
                            },
                            'P' => {
                                piece = Some(ChessPiece {
                                    piece: Piece::Pawn,
                                    color: Color::White,
                                })
                            },
                            'R' => {
                                piece = Some(ChessPiece {
                                    piece: Piece::Rook,
                                    color: Color::White,
                                })
                            },
                            'N' => {
                                piece = Some(ChessPiece {
                                    piece: Piece::Knight,
                                    color: Color::White,
                                })
                            },
                            'B' => {
                                piece = Some(ChessPiece {
                                    piece: Piece::Bishop,
                                    color: Color::White,
                                })
                            },
                            'Q' => {
                                piece = Some(ChessPiece {
                                    piece: Piece::Queen,
                                    color: Color::White,
                                })
                            },
                            'K' => {
                                piece = Some(ChessPiece {
                                    piece: Piece::King,
                                    color: Color::White,
                                })
                            },
                            e => {
                                // dbg!(e);
                                panic!("invalid char in fen board");
                            }
                        };
                        pieces_v.push(piece);
                    },
                    Some(n) => {
                        for _ in 0..n { // empty square N times
                            pieces_v.push(None);
                        }
                    }
                }
            }
            pieces_matrix.push(pieces_v);
        }

        let ranks_v: Vec<[Option<ChessPiece>; 8]> = pieces_matrix.into_iter()
        .map(|rank| {
            rank.try_into().unwrap()
        })
        .collect();

        ranks_v.try_into().expect("error converting vec into array/slice")
    }

    fn en_passant_from_fen(fen_parts: Vec<&str>) -> Option<Square> {
        let en_passant = fen_parts.get(3)
        .expect("fen string should come with en passant encoded") // get the element at index 3 if not out of bounds
        .trim();

        match en_passant {
            "-" => None,
            s => {
                let square = Square::new( // None if it is out of the board
                    s.chars().nth(0).expect("fen should not be empty"),
                s.chars().nth(1).expect("fen should have the target square, and the square must have a rank as well").to_digit(10).unwrap().try_into().unwrap()
                );
                match square {
                    None => None,
                    Some(e) => {
                        if e.rank == 3 || e.rank == 6 {
                            square
                        } else {
                            None
                        }
                    }
                }
            }
        }
    }

    // removes all castling rights because the King just moved
    pub fn remove_all_castling(&mut self, color: &Color) {
        match color {
            Color::Black => {
                for c in &self.castling_rights.clone() {
                    match c {
                        'k' | 'q' => {
                            self.castling_rights.retain(|&r|  r != *c); // remove that char from the vec
                        },
                        _ => continue,
                    }
                }
            },
            Color::White => {
                for c in &self.castling_rights.clone() {
                    match c {
                        'K' | 'Q' => {
                            self.castling_rights.retain(|&r|  r != *c); // remove that char from the vec
                        },
                        _ => continue,
                    }
                }
            },
        }
    }

    // a rook moved, remove castling from that side
    pub fn remove_castling(&mut self, moving_color: &Color, initial_square: &Square) {
        match moving_color {
            Color::Black => {
                if initial_square.rank == 8 && initial_square.file == 'h' { // king side remove
                    self.castling_rights.retain(|c| *c != 'k');
                } else if initial_square.rank == 8 && initial_square.file == 'a' { // queen side remove
                    self.castling_rights.retain(|c| *c != 'q');
                }
            }
            Color::White => {
                if initial_square.rank == 1 && initial_square.file == 'h' { // king side remove
                    self.castling_rights.retain(|c| *c != 'K');
                } else if initial_square.rank == 1 && initial_square.file == 'a' { // queen side remove
                    self.castling_rights.retain(|c| *c != 'Q');
                }
            }
        }
    }

    pub fn en_passant(&mut self, square: Option<Square>) { // changes the en-passant target square
        let previous_en_passant = self.en_passant;
        
        match previous_en_passant {
            None => {},
            Some(s) => {
                let ghost_color = if s.rank == 3 { Color::White } else { Color::Black };
                let piece_at_ghost = self.get_piece_at_square(&s);

                match piece_at_ghost {
                    None => {
                        self.update_square(None, &s); // remove the temporary en passant target piece
                    },
                    Some(p) => {
                        if p.color == ghost_color {
                            self.update_square(None, &s); // remove the temporary en passant target piece
                        }
                    }
                }

                
            },
        };

        match square {
            None => {},
            Some(s) => {
                let color = if s.rank == 3 { Color::White } else { Color::Black };
                
                match s.rank {
                    3 | 6 => { // or
                        self.update_square(Some(ChessPiece {
                            color: color,
                            piece: Piece::Pawn,
                        }), &s);
                    },
                    _ => { panic!("ilegal en passant target square") }
                };
            }
        };

        self.en_passant = square;
    }

    pub fn get_en_passant(&self) -> Option<Square> {
        self.en_passant
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

    pub fn pseudo_legal_moves(&self, moving_player: &Color) -> Vec<Move> {
        let mut legal_moves = vec![];

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
                        
                            self.insert_valid_moves(piece, &pos, &mut legal_moves, moving_player); //
                        }
                    }
                }
            }
        };

        self.add_castling(moving_player, &mut legal_moves);

        legal_moves
    }

    //inserts the valid moves for one piece
    fn insert_valid_moves(&self, piece: &ChessPiece, pos: &Square, valid_moves: &mut Vec<Move>, color: &Color) {
        let all_moves = piece.all_moves(pos); // all moves for the piece, including invalid ones   
        
        for m in all_moves {
            if self.is_move_valid(m, color) { // check if the piece move is valid within the board context
                match piece.piece {
                    Piece::Pawn => { // convert moves to promotions
                        self.convert_promotions(m, valid_moves);
                    }
                    _ => {
                        valid_moves.push(m);
                    }
                }
                // println!("b")
            }
        };
    } 

    fn is_move_valid(&self, m: Move, color: &Color) -> bool {
        let starting_square = &m.starting_square();
        let final_square = &m.final_square();


        let ep = match self.en_passant {
            None => false,
            Some(_) => true,
        };
        let is_ep_ghost = match self.en_passant {// check if the final square matches the en-passant target square
            None => { false },
            Some(s) => { if s == *final_square { true } else { false }}
        };

        let moving_piece = self.get_piece_at_square(starting_square).expect("there should be a piece at this position in the board");

        let final_square_piece=  self.get_piece_at_square(final_square); // should be none if there is no piece at the final square

        let rank_dif = final_square.rank - starting_square.rank;
        let file_dif = (final_square.file as i8) - (starting_square.file as i8);

        let is_vertical = if rank_dif == 0 { false } else { true };

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
                        None => return false, //only if the square is outside the board, never triggers in practice
                        Some(s) => if self.get_piece_at_square(&s) != None && (if ep { self.en_passant.unwrap() != s } else {true})  {
                            return false;
                        }
                    }
                }
            },

            Piece::Rook => {
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
                        Some(s) => if self.get_piece_at_square(&s) != None && (if ep { self.en_passant.unwrap() != s } else {true}) {
                            return false;
                        }
                        None => { //only if the square is outside the board, never triggers in practice
                            return false;
                        }
                    }
                }
            },
            
            Piece::Queen => {
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
                            None => return false, //only if the square is outside the board, never triggers in practice
                            Some(s) => if self.get_piece_at_square(&s) != None && (if ep { self.en_passant.unwrap() != s } else {true}) {
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
                            Some(s) => if self.get_piece_at_square(&s) != None && (if ep { self.en_passant.unwrap() != s } else {true}) {
                                return false;
                            }
                            None => {
                                return false; //only if the square is outside the board, never triggers in practice
                            }
                        }
                    }
                    
                }
            }

            /*
            Piece::Pawn => { // convert moves that are promotions to promotions, because by default the mov gen does not do it in piece.rs
                let final_rank = m.final_square().rank;

                if final_rank == 1 { // black promotion

                } else if final_rank == 8 { // white promotion
                    
                }
            },*/

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
                                        if self.get_piece_at_square(&Square { rank: starting_square.rank - 1, file: starting_square.file }) != None {
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
                                        if self.get_piece_at_square(&Square { rank: starting_square.rank + 1, file: starting_square.file }) != None {
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
                    if is_ep_ghost { // check for the ghost target
                        true
                    } else {
                        false
                    }
                } else { // é capturável/da outra equipa
                    if is_ep_ghost {
                        match moving_piece.piece {
                                Piece::Pawn => {
                                    if starting_square.file != final_square.file {
                                        true
                                    } else {
                                        false
                                    }
                                },
                                _ => true, // enemy pieces can move there because it is actually an empty square, there is no piece there
                            }
                    } else {
                        match piece.piece {
                            Piece::King => {
                                // println!("falta a logica de cheque do rei");
                                match moving_piece.piece {
                                    Piece::Pawn => { 
                                        if starting_square.file != final_square.file {
                                            true
                                        } else {
                                            false
                                        }
                                    } // so that the pawn cant capture the by moving forward
                                    _ => true
                                }
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
                                        // TODO!!
                                        // println!("falta a logica de peças capturarem-se");
                                        true
                                    }
                            }
                        }
                    }
                }
            },
        }
    }

    fn add_castling(&self, color: &Color, legal_moves: &mut Vec<Move>) {
        match color {
            Color::Black => {
                for c in &self.castling_rights {
                    match c {
                        'k' => { // if the kings path is not in check -> the square f8, and if there are no pieces in f8 and g8
                            if self.get_piece_at_square(&Square { rank: 8, file: 'f' }) != None {
                                continue;
                            } else if self.get_piece_at_square(&Square { rank: 8, file: 'g' }) != None {
                                continue;                                
                            } else {
                                legal_moves.push(Move::from_squares(Square { rank: 8, file: 'e' }, Square { rank: 8, file: 'g' }, None));
                            }
                        }
                        'q' => { // if the kings path is not in check -> the square d8, and if there are no pieces in d8, c8 and b8
                            if self.get_piece_at_square(&Square { rank: 8, file: 'd' }) != None {
                                continue;
                            } else if self.get_piece_at_square(&Square { rank: 8, file: 'c' }) != None {
                                continue;                                
                            } else if self.get_piece_at_square(&Square { rank: 8, file: 'b' }) != None {
                                continue;
                            } else {
                                legal_moves.push(Move::from_squares(Square { rank: 8, file: 'e' }, Square { rank: 8, file: 'c' }, None));
                            }
                        }
                        _ => {}
                    }
                }
            }
            Color::White => {
                for c in &self.castling_rights {
                    match c {
                        'K' => { // if the kings path is not in check -> the square f1(remove_checks handles this in engine.rs),
                            //  and if there are no pieces in f1 and g1
                            if self.get_piece_at_square(&Square { rank: 1, file: 'f' }) != None {
                                continue;
                            } else if self.get_piece_at_square(&Square { rank: 1, file: 'g' }) != None {
                                continue;                                
                            } else {
                                legal_moves.push(Move::from_squares(Square { rank: 1, file: 'e' }, Square { rank: 1, file: 'g' }, None));
                            }
                        }
                        'Q' => { // if the kings path is not in check -> the square d1, and if there are no pieces in d1, c1 and b1
                            if self.get_piece_at_square(&Square { rank: 1, file: 'd' }) != None {
                                continue;
                            } else if self.get_piece_at_square(&Square { rank: 1, file: 'c' }) != None {
                                continue;                                
                            } else if self.get_piece_at_square(&Square { rank: 1, file: 'b' }) != None {
                                continue;
                            } else {
                                legal_moves.push(Move::from_squares(Square { rank: 1, file: 'e' }, Square { rank: 1, file: 'c' }, None));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn is_a_castle(&self, m: Move) -> bool {
        let moving_piece = self.get_piece_at_square(&m.starting_square());

        match moving_piece {
            None => {
                println!("PANICC should be moving a piece");
                panic!("should be moving a piece");
            },
            Some(p) => {
                if p.piece != Piece::King {
                    return false;
                }
            }
        }

        let starting_square = m.starting_square();
        let starting_file = starting_square.file;
        let starting_rank = starting_square.rank;

        let final_square = m.final_square();
        let final_file = final_square.file;
        let final_rank = final_square.rank;

        if starting_rank == 1 && final_rank == 1 && starting_file == 'e' && final_file == 'g' {
            true
        } else if starting_rank == 1 && final_rank == 1 && starting_file == 'e' && final_file == 'c' {
            true
        } else if starting_rank == 8 && final_rank == 8 && starting_file == 'e' && final_file == 'g' {
            true
        } else if starting_rank == 8 && final_rank == 8 && starting_file == 'e' && final_file == 'c' {
            true
        } else {
            false
        }
    }

    fn convert_promotions(&self, m: Move, valid_moves: &mut Vec<Move>) {
        let final_rank = m.final_square().rank;

        if final_rank == 1 || final_rank == 8 { // it is a promotion
            valid_moves.push(Move::from_squares(m.starting_square(), m.final_square(), Some(Promotion::Queen)));
            valid_moves.push(Move::from_squares(m.starting_square(), m.final_square(), Some(Promotion::Rook)));
            valid_moves.push(Move::from_squares(m.starting_square(), m.final_square(), Some(Promotion::Bishop)));
            valid_moves.push(Move::from_squares(m.starting_square(), m.final_square(), Some(Promotion::Knight)));
        } else { // just a normal pawn move
            valid_moves.push(m);
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

pub fn file_to_num(c: char) -> u8 {
    if ('a'..='h').contains(&c) {
        c as u8 - b'a'          // 0..=7
    } else {
        panic!("invalid file");
    }
}

pub fn num_to_file(n: u8) -> char {
    if n < 8 {
        (b'a' + n) as char     // 0..=7
    } else {
        panic!("invalid file index");
    }
}
