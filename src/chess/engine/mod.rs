use core::panic::PanicInfo;
use core::time;
use std::io::{Write, stdout};
use std::thread::sleep;
use std::time::Instant;
use std::{clone, i32};
use std::{fmt::DebugStruct, mem::transmute, vec};

use crate::chess::board;
use crate::chess::move_square::Promotion;
use crate::chess::move_square::{Move, Square};
use crate::chess::piece::{ChessPiece, Piece};
use board::Board;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};

use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};

#[derive(PartialEq, Debug, Clone, Copy, Default)]
pub enum Color {
    #[default]
    White, // white is the default, but it does not matter, it is just to initialize pieces in the Zobrist table
    Black,
}

impl Color {
    fn other(self) -> Self {
        // returns the oposite color
        if self == Color::Black {
            Color::White
        } else {
            Color::Black
        }
    }
}

pub enum GameState {
    Playing, // created and started (ongoing game)
    CheckMate(Color),
    StaleMate,
    Created, // new game created but not started
}

pub struct PlayerTimes {
    // in miliseconds
    pub wtime: i32,
    pub btime: i32,
    pub winc: i32,
    pub binc: i32,
    pub movetime: i32,
}

#[derive(Default, Debug, Clone)]
pub struct ZobristPieceKey {
    piece: ChessPiece,
    pub key: u64,
}

#[derive(Debug, Clone)]
pub struct ZobristKeys {
    // initial random keys for representing a hash/unique board position
    pub piece_table: [[[ZobristPieceKey; 12]; 8]; 8],
    pub blacks_turn: u64,
    pub castling_keys: [u64; 4],   // all 4 castling rights
    pub en_passant_keys: [u64; 8], // the files for en passant
}
/// **this renders as bold text in markdown in cargo doc**
pub struct Engine {
    pub is_running: bool, // this is the only field that apears in the cargo doc API documentation, because it is public
    game_state: GameState,
    color: Color,
    current_player: Color,
    board: Board,
    legal_moves: Vec<Move>,

    /* a hash or digest is a fixed size value which is the output of a hash function
     a hash function takes an input of arbitrary size (normally called key) and returns a fixed size output (the hash)

     A good hash function has two properties:
     Deterministic: same input always produces the same output
     Avalanche effect: a tiny change in input (moving one pawn) produces a completely different output. This makes it hard to revert the hash to get the key that originated it

     a hash table is just a table represented by an array where the index is just the hash.
     the remainder is a common method to map integer hashes to the array in the hash table:
     index = hash(key) % array_size
     table[index] = value

     there are 4.8 x 10^44 possible legal chess positions, but only 1.8 x 10^19 possible zobrist hashes for representing all boards, so there is a very very small chance that two boards colide and have the same zobrist hash (not desirable)


    table with the initially generated random zobrist keys, 12, one key for each piece type distinguishing color in each of the 64 squares of the board
    */
    zobrist_keys: ZobristKeys,
    board_history: Vec<u64>, // every board position played up to this point in this game
}

impl Engine {
    pub fn new() -> Self {
        let keys = Self::generate_zobrist_keys();
        Engine {
            is_running: true,
            game_state: GameState::Created,
            color: Color::Black,
            current_player: Color::White,
            board: Board::new(keys.clone()),
            legal_moves: vec![],
            zobrist_keys: keys,
            board_history: vec![],
        }
    }

    fn generate_zobrist_keys() -> ZobristKeys {
        let mut table: [[[ZobristPieceKey; 12]; 8]; 8] = Default::default(); // initializes with meaningless default values which get overwritten anyways

        //DONT CHANGE THE ORDER OF THIS VEC BELLOW
        let all_piece_types = vec![
            ChessPiece {
                color: Color::Black,
                piece: Piece::Pawn,
            },
            ChessPiece {
                color: Color::Black,
                piece: Piece::Knight,
            },
            ChessPiece {
                color: Color::Black,
                piece: Piece::Bishop,
            },
            ChessPiece {
                color: Color::Black,
                piece: Piece::Queen,
            },
            ChessPiece {
                color: Color::Black,
                piece: Piece::King,
            },
            ChessPiece {
                color: Color::Black,
                piece: Piece::Rook,
            },
            ChessPiece {
                color: Color::White,
                piece: Piece::Pawn,
            },
            ChessPiece {
                color: Color::White,
                piece: Piece::Knight,
            },
            ChessPiece {
                color: Color::White,
                piece: Piece::Bishop,
            },
            ChessPiece {
                color: Color::White,
                piece: Piece::Queen,
            },
            ChessPiece {
                color: Color::White,
                piece: Piece::King,
            },
            ChessPiece {
                color: Color::White,
                piece: Piece::Rook,
            },
        ];

        // using the same seed every game results in the same sequence of keys generated after, which is what we want:
        // it allows for opening books, because saving hashes to the disk means the keys are still usefull afterways, because the hashes representing the same position are equal in diferent games and processes in the computer:
        // persistent transposition tables saved between games: good for opening teory, not so much for games because it is impossible for the same chess game to get played again
        let mut rng = StdRng::seed_from_u64(123456789876543210); // the number argument is a seed, could be anything

        for row in 0..8 {
            for col in 0..8 {
                for (k, piece) in all_piece_types.iter().enumerate() {
                    table[row][col][k] = ZobristPieceKey {
                        piece: *piece,
                        key: rng.random(), // always the same sequence of random keys
                    };
                }
            }
        }

        let blacks_turn: u64 = rng.random(); // this key is xored into the zobrish hash when its blacks turn, and not shored when its not black

        /* castling
        At any point in the game, there are 4 castling rights that can independently be true or false:
        White kingside
        White queenside
        Black kingside
        Black queenside
        we generate a random u64 for each.
        Whenever a castling right is active, we XOR its number into the hash.
        When the right is lost (king or rook moves), we XOR it back out to remove it.
        */
        let white_kingside: u64 = rng.random();
        let white_queenside: u64 = rng.random();
        let black_kingside: u64 = rng.random();
        let black_queenside: u64 = rng.random();

        let castling_keys = [
            white_kingside,
            white_queenside,
            black_kingside,
            black_queenside,
        ];

        /*
        En passant is only possible on specific files (a through h), so we only need 8 keys, one per file.
        When an en passant capture is available, we XOR in the key for that file.
        When it's no longer available (next move), we XOR it back out.
        we do not need to encode the rank because en passant always happens on rank 3 or 6, which is determined by whose turn it is.
         */
        let ep_a: u64 = rng.random();
        let ep_b: u64 = rng.random();
        let ep_c: u64 = rng.random();
        let ep_d: u64 = rng.random();
        let ep_e: u64 = rng.random();
        let ep_f: u64 = rng.random();
        let ep_g: u64 = rng.random();
        let ep_h: u64 = rng.random();

        let ep_keys = [ep_a, ep_b, ep_c, ep_d, ep_e, ep_f, ep_g, ep_h];

        ZobristKeys {
            piece_table: table,
            blacks_turn: blacks_turn,
            castling_keys: castling_keys,
            en_passant_keys: ep_keys,
        }
    }

    pub fn clear(&mut self) {
        self.board_history.clear();
    }

    pub fn load_from_fen(&mut self, fen_parts: Vec<&str>) {
        // is called every round if the game started from fen
        self.board = Board::from_fen(fen_parts, self.zobrist_keys.clone());
        self.current_player = self.board.current_player;
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn close(&mut self) {
        self.is_running = false;
    }

    fn update_active_player(&mut self) {
        // this function is not rendered in the docs (cargo doc) because it is private and not part of the API
        self.board.set_current_player_in_fen(&self.current_player);
        if let Color::Black = self.current_player {
            self.current_player = Color::White;
        } else {
            self.current_player = Color::Black;
        }
    }

    pub fn start(&mut self) {
        self.game_state = GameState::Playing;
        // todo!()
    }

    pub fn game_state(&self) -> &GameState {
        &self.game_state
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_active_player(&self) -> &Color {
        &self.current_player
    }

    pub fn has_started(&self) -> bool {
        match &self.game_state {
            GameState::Created => false,
            GameState::Playing => true,
            GameState::CheckMate(c) => true,
            GameState::StaleMate => true,
        }
    }

    pub fn is_legal(&mut self, m: &Move) -> bool {
        // dbg!(&self.color_playing);
        self.legal_moves = self.board.pseudo_legal_moves(&self.current_player);

        // println!("num of legal moves: {}", self.legal_moves.len());
        // dbg!(m);

        if self.legal_moves.contains(m) {
            true
        } else {
            false
        }
    }

    fn promote_piece(
        moving_piece: &mut Option<ChessPiece>,
        m: &Move,
        board: &mut Board,
        current_player: Color,
    ) {
        // promotes a piece if it is a pawn and is promoting in the move
        match m.promotion {
            None => {
                *moving_piece = board.get_piece_at_square(&m.starting_square()); // none if the square is empty
            }
            Some(promotion) => {
                *moving_piece = Some(ChessPiece {
                    color: current_player,
                    piece: match promotion {
                        Promotion::Bishop => Piece::Bishop,
                        Promotion::Knight => Piece::Knight,
                        Promotion::Rook => Piece::Rook,
                        Promotion::Queen => Piece::Queen,
                    },
                })
            }
        };
    }

    //  pre: self.is_legal(m)
    fn move_piece(&mut self, m: &Move) {
        let mut moving_piece = None;

        let starting_square = m.starting_square();
        let final_square = m.final_square();

        Self::promote_piece(&mut moving_piece, m, &mut self.board, self.current_player);

        let final_piece = self.board.get_piece_at_square(&final_square);

        let is_a_castle = self.board.is_a_castle(*m);
        // dbg!(&self.board);
        self.board.update_square(moving_piece, &final_square);

        // dbg!(&self.board);
        self.board.update_square(None, &starting_square);

        if is_a_castle {
            Self::move_castled_rook(&mut self.board, &final_square);
        }

        Self::update_en_passant(m, &mut self.board, moving_piece, final_piece);
        Self::update_castling_rights(
            &mut self.board,
            moving_piece,
            final_piece,
            &starting_square,
            &final_square,
        );

        self.board.hash_blacks_turn();

        self.update_active_player();

        self.board_history.push(self.board.zobrist_hash);
    }

    fn simulate_move(&self, m: &Move, board: &mut Board, color: &Color) {
        let mut moving_piece = None;

        let starting_square = m.starting_square();
        let final_square = m.final_square();

        Self::promote_piece(&mut moving_piece, m, board, *color);

        let final_piece = board.get_piece_at_square(&final_square);

        let is_a_castle = board.is_a_castle(*m);

        board.update_square(moving_piece, &final_square);

        board.update_square(None, &starting_square);

        if is_a_castle {
            Self::move_castled_rook(board, &final_square);
        }

        Self::update_en_passant(m, board, moving_piece, final_piece);
        Self::update_castling_rights(
            board,
            moving_piece,
            final_piece,
            &starting_square,
            &final_square,
        );

        board.hash_blacks_turn();
    }

    fn move_castled_rook(board: &mut Board, final_square: &Square) {
        // m is the kings castling move notation
        // let is_king_side = if engine_final_square == Square::new('g', 8).unwrap() { true } else { false };
        // let is_queen_side = if engine_final_square == Square::new('c', 8).unwrap() { true } else { false };
        if final_square.rank == 1 {
            // white
            if final_square.file == 'g' {
                // kingside, rook moves from h1 to f1
                board.update_square(
                    Some(ChessPiece {
                        piece: Piece::Rook,
                        color: Color::White,
                    }),
                    &Square { rank: 1, file: 'f' },
                );
                board.update_square(None, &Square { rank: 1, file: 'h' });
            } else if final_square.file == 'c' {
                // queenside, rook moves from a1 to d1
                board.update_square(
                    Some(ChessPiece {
                        piece: Piece::Rook,
                        color: Color::White,
                    }),
                    &Square { rank: 1, file: 'd' },
                );
                board.update_square(None, &Square { rank: 1, file: 'a' });
            } else {
                println!("PANNICCC, should be a castling move");
                panic!("should be a castling move");
            }
        } else if final_square.rank == 8 {
            // black
            if final_square.file == 'g' {
                // kingside, rook moves from h8 to f8
                board.update_square(
                    Some(ChessPiece {
                        piece: Piece::Rook,
                        color: Color::Black,
                    }),
                    &Square { rank: 8, file: 'f' },
                );
                board.update_square(None, &Square { rank: 8, file: 'h' });
            } else if final_square.file == 'c' {
                // queenside, rook moves from a8 to d8
                board.update_square(
                    Some(ChessPiece {
                        piece: Piece::Rook,
                        color: Color::Black,
                    }),
                    &Square { rank: 8, file: 'd' },
                );
                board.update_square(None, &Square { rank: 8, file: 'a' });
            } else {
                println!("PANNICCC, should be a castling move");
                panic!("should be a castling move");
            }
        } else {
            println!("PANNICCC, should be a castling move");
            panic!("should be a castling move");
        }
    }

    fn update_castling_rights(
        board: &mut Board,
        moving_piece: Option<ChessPiece>,
        final_piece_before_move: Option<ChessPiece>,
        starting_square: &Square,
        final_square: &Square,
    ) {
        match moving_piece {
            Some(p) => {
                match p.piece {
                    Piece::King => {
                        // king moves
                        board.remove_all_castling(&p.color);
                    }
                    Piece::Rook => {
                        // friendly rook moves
                        board.remove_castling(&p.color, starting_square);
                    }
                    _ => {}
                }
            }
            None => {
                println!("PANNNNICCCCCCCCCC");
                panic!("moving no piece");
            }
        };

        match final_piece_before_move {
            None => {}
            Some(p) => {
                // if a friendly rook is captured on its home square, no castling can happen on that side
                if p.piece == Piece::Rook {
                    match p.color {
                        // the color of the rook being captured
                        Color::Black => {
                            if final_square.rank == 8
                                && (final_square.file == 'a' || final_square.file == 'h')
                            {
                                board.remove_castling(&Color::Black, final_square);
                            }
                        }
                        Color::White => {
                            if final_square.rank == 1
                                && (final_square.file == 'a' || final_square.file == 'h')
                            {
                                board.remove_castling(&Color::White, final_square);
                            }
                        }
                    }
                } else if p.piece == Piece::King {
                    // needed because in deep searches, the king is captured, and you cant try to castle and move an empty square
                    board.remove_all_castling(&p.color);
                }
            }
        }
    }

    fn update_en_passant(
        m: &Move,
        board: &mut Board,
        moving_piece: Option<ChessPiece>,
        final_piece_before_move: Option<ChessPiece>,
    ) {
        // static method to avoid borrowing problems
        let starting_square = m.starting_square();
        let final_square = m.final_square();

        let rank_dif = (final_square.rank - starting_square.rank).abs();
        let file_dif = (final_square.file as i8 - starting_square.file as i8).abs();

        let is_moving_piece_pawn = moving_piece
            .map(|p| p.piece == Piece::Pawn)
            .unwrap_or(false);
        let is_double_push_white =
            is_moving_piece_pawn && starting_square.rank == 2 && rank_dif == 2;
        let is_double_push_black =
            is_moving_piece_pawn && starting_square.rank == 7 && rank_dif == 2;
        let is_en_passant = is_double_push_white || is_double_push_black;

        let is_white_capture = starting_square.rank == 5; // white capture black en passant

        // let final_piece = board.get_piece_at_square(&m.final_square());

        match final_piece_before_move {
            // to check if it is a capture, to both pawns (ghost target and the actual pawn)
            None => {}
            Some(p) => {
                let e_p = board.get_en_passant();

                match e_p {
                    None => {}
                    Some(s) => {
                        //its en passant
                        if final_square == s
                            && (moving_piece.unwrap().piece == Piece::Pawn)
                            && file_dif == 1
                        {
                            // we also have to remove the original pawn that moved 2 squares
                            board.update_square(
                                None,
                                &Square::new(
                                    s.file,
                                    if is_white_capture {
                                        s.rank - 1
                                    } else {
                                        s.rank + 1
                                    },
                                )
                                .unwrap(),
                            );
                        }
                    }
                }
            }
        };

        if is_double_push_white {
            // update the target square
            board.en_passant(Square::new(starting_square.file, starting_square.rank + 1));
        } else if is_double_push_black {
            board.en_passant(Square::new(starting_square.file, starting_square.rank - 1));
        } else {
            board.en_passant(None);
        }
    }

    //Called from main.rs after Position command
    pub fn apply_moves(&mut self, moves: Vec<String>, is_fen: bool) {
        if !is_fen {
            self.board = Board::new(self.zobrist_keys.clone()); // not in case of fen, because load_from_fen() is called from main before this method.
            self.current_player = self.board.current_player;
            self.board_history.clear();
        }

        for move_before in &moves {
            let m = Move::from_uci(move_before).unwrap();
            self.move_piece(&m); // updates board and active player color aswell
        }

        /*
        if !moves.is_empty() {
            let last_move = moves.last();
            self.add_fen(moves.last().unwrap());
        }*/
    }

    /* let a = 0b1011;
     println!("{}", a);   // prints: 11    (decimal)
     println!("{:b}", a); // prints: 1011  (binary)
     println!("{:x}", a); // prints: b     (hexadecimal)
     println!("{:o}", a); // prints: 13    (octal)
    */

    fn get_legal_moves(&self, board: &Board, color: Color) -> Vec<Move> {
        let mut pseudo_legal_moves = board.pseudo_legal_moves(&color);
        self.remove_checks(&mut pseudo_legal_moves, color, board);

        /*println!("Legal moves for {:?}:", color);
        for m in &pseudo_legal_moves {
            println!("  {}", m.to_uci());
        } */

        pseudo_legal_moves // become legal after removing moves that leave the king in check
    }

    fn pawn_attacks(board: &Board, color: &Color) -> Vec<Square> {
        // needed because of tricky castle and pawn captures interactions in remove_checks()
        let mut attacked = vec![];
        for (i, row) in board.get_pieces().iter().enumerate() {
            for (j, piece) in row.iter().enumerate() {
                if let Some(p) = piece {
                    if p.color == *color && p.piece == Piece::Pawn {
                        let file = board::num_to_file(j as u8); // j is the file index (0..7) -> (a..=h)
                        let rank = (i as i8) + 1; // i is the rank index (0..7 → 1..8)

                        let pos = Square::new(file, rank).unwrap();

                        match color {
                            Color::Black => {
                                match pos.offset(1, -1) {
                                    // returns None if the pos is outside the board
                                    None => {}
                                    Some(s) => {
                                        attacked.push(s);
                                    }
                                };
                                match pos.offset(-1, -1) {
                                    // returns None if the pos is outside the board
                                    None => {}
                                    Some(s) => {
                                        attacked.push(s);
                                    }
                                };
                            }
                            Color::White => {
                                match pos.offset(1, 1) {
                                    // returns None if the pos is outside the board
                                    None => {}
                                    Some(s) => {
                                        attacked.push(s);
                                    }
                                };
                                match pos.offset(-1, 1) {
                                    // returns None if the pos is outside the board
                                    None => {}
                                    Some(s) => {
                                        attacked.push(s);
                                    }
                                };
                            }
                        }
                    }
                }
            }
        }
        attacked
    }

    fn remove_checks(&self, legal_moves: &mut Vec<Move>, color: Color, board: &Board) {
        // removes moves that make the engines king remain in check
        for m in legal_moves.clone() {
            // for each of the engine legal moves, check if it does not leave the engines king in check
            let mut board_clone = board.clone();
            let is_a_castle = board_clone.is_a_castle(m);

            self.simulate_move(&m, &mut board_clone, &color);

            let other_player = if color == Color::Black {
                Color::White
            } else {
                Color::Black
            };
            let other_player_legal_moves = board_clone.pseudo_legal_moves(&other_player);

            for other_m in other_player_legal_moves {
                let final_square = other_m.final_square();

                if is_a_castle {
                    // remove castles when the king is in check and its path
                    let engine_final_square = m.final_square();
                    match color {
                        Color::Black => {
                            let is_king_side =
                                if engine_final_square == Square::new('g', 8).unwrap() {
                                    true
                                } else {
                                    false
                                };
                            let is_queen_side =
                                if engine_final_square == Square::new('c', 8).unwrap() {
                                    true
                                } else {
                                    false
                                };

                            let pawn_attacks = Self::pawn_attacks(&board_clone, &other_player); // because board_clone.pseudo_legal_moves(&other_player); does not generate pawn checks, since simulate move already castled the king
                            if pawn_attacks.contains(&Square::new('e', 8).unwrap()) {
                                legal_moves.retain(|mov| mov != &m);
                            }

                            if final_square == Square::new('e', 8).unwrap() {
                                // the king cant castle to escape a check
                                legal_moves.retain(|mov| mov != &m);
                            }

                            if final_square == Square::new('f', 8).unwrap() && is_king_side {
                                // check for the kings path when castling
                                legal_moves.retain(|mov| mov != &m);
                            } else if final_square == Square::new('d', 8).unwrap() && is_queen_side
                            {
                                legal_moves.retain(|mov| mov != &m);
                            }
                        }
                        Color::White => {
                            let is_king_side =
                                if engine_final_square == Square::new('g', 1).unwrap() {
                                    true
                                } else {
                                    false
                                };
                            let is_queen_side =
                                if engine_final_square == Square::new('c', 1).unwrap() {
                                    true
                                } else {
                                    false
                                };

                            let pawn_attacks = Self::pawn_attacks(&board_clone, &other_player); // because board_clone.pseudo_legal_moves(&other_player); does not generate pawn checks, since simulate move already castled the king
                            if pawn_attacks.contains(&Square::new('e', 1).unwrap()) {
                                legal_moves.retain(|mov| mov != &m);
                            }

                            if final_square == Square::new('e', 1).unwrap() {
                                // the king cant castle to escape a check
                                legal_moves.retain(|mov| mov != &m);
                            }

                            if final_square == Square::new('f', 1).unwrap() && is_king_side {
                                // check for the kings path when castling
                                legal_moves.retain(|mov| mov != &m);
                            } else if final_square == Square::new('d', 1).unwrap() && is_queen_side
                            {
                                legal_moves.retain(|mov| mov != &m);
                            }
                        }
                    };
                }

                let final_square_piece = board_clone.get_piece_at_square(&final_square);

                match final_square_piece {
                    None => {} // move into an empty Square
                    Some(other) => {
                        if other.piece == Piece::King {
                            // engines king can be captured the next move (he will be in check, so the move is ilegal)
                            // println!("  -> {} attacks king via {}, removing it", other_m.to_uci(), m.to_uci());
                            legal_moves.retain(|mov| mov != &m);
                        }
                    }
                }
            }
        }
    }

    pub fn web_perft(&self, depth: i32) {
        println!("starting webperft {depth}...");
        let mut board_clone = self.board.clone();

        for d in 1..=depth {
            // iterative deepening
            if d != depth {
                // to remove iterative deepening
                continue;
            }
            let start = std::time::Instant::now();

            //
            let possible_positions = self.perft_aux_wp(d, &mut board_clone, self.current_player);
            let duration = start.elapsed(); // not acurate because perft_aux does iterative deepening, and it does not account for time of previous

            // moves generated per_second
            let nodes_per_second = possible_positions as f64 / duration.as_secs_f64();

            // if d == depth {
            // println!("{possible_positions} possible positions at depth {d} generated in {:.3} seconds, ({:.0} nodes per second)", duration.as_secs_f64(), nodes_per_second);
            // }
            println!(
                "{possible_positions} possible positions at depth {d} generated in {:.3} seconds, ({:.0} nodes per second)",
                duration.as_secs_f64(),
                nodes_per_second
            );
        }
        println!("------------------- finished webperft ---------------------");
    }

    fn perft_aux_wp(&self, depth: i32, board: &mut Board, color: Color) -> i32 {
        //max depth this iteration
        if depth == 0 {
            return 1;
        }

        let mut nodes = 0;
        let next_color = if color == Color::White {
            Color::Black
        } else {
            Color::White
        };

        for m in self.get_legal_moves(board, color) {
            let mut board_for_this_branch = board.clone();

            self.simulate_move(&m, &mut board_for_this_branch, &color);

            // let branching_factor = self.get_legal_moves(&board_for_this_branch, next_color).len();

            //move count for the specific move just made. it is the number of possible positions/nodes after this move
            let move_count = self.perft_aux(depth - 1, &mut board_for_this_branch, next_color);

            nodes += move_count;

            // println!("{}: {move_count} possible moves after it when depth = {depth}", m.to_uci());//understanding

            println!("{}: {move_count}", m.to_uci()); // for webperft diff
        }
        // this bellow makes sense because depth 1 means only 1 move is allowed, and that move is b1a3, which leads to a leaf node and implies that the only position allowed is the one after b1a3 is played
        // Move { initial: Square { rank: 1, file: 'b' }, end: Square { rank: 3, file: 'a' } }: 1 possible moves after it when depth = 1
        return nodes;
    }

    pub fn perft(&self, max_depth: i32) {
        println!("starting perft {max_depth}...");
        let mut board_clone = self.board.clone();

        for depth in 1..=max_depth {
            // iterative deepening

            let start = std::time::Instant::now();

            //
            let possible_positions = self.perft_aux(depth, &mut board_clone, self.current_player);
            let duration = start.elapsed(); // not acurate because perft_aux does iterative deepening, and it does not account for time of previous

            // moves generated per_second
            let nodes_per_second = possible_positions as f64 / duration.as_secs_f64();

            println!(
                "{possible_positions} possible positions at depth {depth} generated in {:.3} seconds, ({:.0} nodes per second)",
                duration.as_secs_f64(),
                nodes_per_second
            );
        }
        println!("------------------- finished perft -------------------");
    }

    /// Runs perft at a single fixed `depth` without iterative deepening.
    ///
    /// Unlike [`perft`](Engine::perft), which runs from depth `1` up to `max_depth`,
    /// this function evaluates **only** the specified depth in a single pass.
    ///
    /// # Arguments
    /// * `depth` - The fixed depth at which to count leaf nodes.
    ///
    /// # Returns
    /// The number of possible positions at the given depth.
    fn unit_perft(&self, depth: i32) -> i32 {
        let mut board_clone = self.board.clone();

        let possible_positions = self.perft_aux(depth, &mut board_clone, self.current_player);

        possible_positions
    }

    fn perft_aux(&self, depth: i32, board: &mut Board, color: Color) -> i32 {
        //max depth this iteration
        if depth == 0 {
            return 1;
        }

        let mut nodes = 0;

        for m in self.get_legal_moves(board, color) {
            let mut board_for_this_branch = board.clone();

            self.simulate_move(&m, &mut board_for_this_branch, &color);

            nodes += self.perft_aux(
                depth - 1,
                &mut board_for_this_branch,
                if color == Color::White {
                    Color::Black
                } else {
                    Color::White
                },
            );
        }

        return nodes;
    }

    // planed time to spend in the next move in ms
    fn calculate_time(&self, times: &PlayerTimes) -> i32 {
        match self.color {
            // always plan thinking there are still 40 moves to go
            Color::Black => {
                let remaining: i32;
                if times.movetime > 0 {
                    return times.movetime;
                } else {
                    remaining = times.btime;
                };
                let time = (remaining / 40) + times.binc / 2; // add the increment
                time.max(100)
            }
            Color::White => {
                let remaining: i32;
                if times.movetime > 0 {
                    return times.movetime;
                } else {
                    remaining = times.wtime;
                };
                let time = (remaining / 40) + times.winc / 2;
                time.max(100)
            }
        }
    }

    fn is_king_in_check(&self, board: &Board, color: &Color) -> bool {
        // check if the king with the given color is in check
        let mut kings_square: Square;
        let is_in_check = false;

        for (i, row) in board.get_pieces().iter().enumerate() {
            for (j, piece) in row.iter().enumerate() {
                match piece {
                    None => {}
                    Some(p) => {
                        if p.piece == Piece::King && &p.color == color {
                            kings_square = Square {
                                rank: (i as i8) + 1,
                                file: board::num_to_file(j as u8),
                            };
                            let oponents_legal_moves = self.get_legal_moves(
                                board,
                                if color == &Color::White {
                                    Color::Black
                                } else {
                                    Color::White
                                },
                            );

                            for m in oponents_legal_moves {
                                if m.final_square() == kings_square {
                                    return true;
                                }
                            }
                            return false;
                        }
                    }
                };
            }
        }

        return is_in_check; // should never reach this return, because the king should be found somewhere in the board
    }

    // in a max node: we define alpha, and test beta
    fn max_value(
        &self,
        depth: i32,
        board: Board,
        start: Instant,
        eng_move_time: i32,
        node_count: &mut u64,
        alpha: i32,
        beta: i32,
        board_history: Vec<u64>,
        ply: i32,
        stop_flag: Arc<AtomicBool>,
        seldepth: &mut i32,
    ) -> i32 {
        *node_count += 1;

        let legal_moves = self.get_legal_moves(&board, Color::White); // child nodes/positions

        if ply > *seldepth {
            *seldepth = ply;
        }

        if legal_moves.is_empty() {
            // if there are no legal moves now, it is checkmate or stalemate
            if self.is_king_in_check(&board, &Color::White) {
                // check if white king was checkmated
                // the ply is to penalize longer mates from the root, and prefer shorter ones
                return -1_000_000 + ply; // Black wins, value close to infinity for normal play: not i32::MIN to not risk overflow
            } else {
                return 0; // stalemate draw
            }
        } else if Self::is_three_fold_draw(board.zobrist_hash, &board_history) {
            return 0;
        } else if depth == 0 {
            if self.has_captures(&board, &legal_moves) {
                //quiescence_search: dont stop searching until all captures are resolved
                return self.quiescence_search_white(
                    board,
                    node_count,
                    alpha,
                    beta,
                    ply,
                    seldepth,
                    start,
                    eng_move_time,
                    stop_flag,
                );
            }
            return self.eval(&board);
        }

        if eng_move_time > 0 {
            if start.elapsed().as_millis() > eng_move_time as u128
                || stop_flag.load(Ordering::Relaxed)
            {
                return self.eval(&board);
            }
        } else if eng_move_time == 0 {
            // go infinite
            if stop_flag.load(Ordering::Relaxed) {
                return self.eval(&board);
            }
        }

        let mut best = i32::MIN;
        let mut alpha = alpha;

        let mut board_history = board_history;

        for m in &legal_moves {
            let mut board_clone = board.clone();
            // let mut board_hist_clone = board_history.clone();

            self.simulate_move(m, &mut board_clone, &Color::White); // do a move

            board_history.push(board_clone.zobrist_hash);

            let v = self.min_value(
                depth - 1,
                board_clone.clone(),
                start,
                eng_move_time,
                node_count,
                alpha,
                beta,
                board_history.clone(),
                ply + 1,
                Arc::clone(&stop_flag),
                seldepth,
            );

            board_history.pop();

            if v > best {
                // for each move that we do, we return the one that is the most valuable for white
                best = v;
            }

            // se o beta (melhor opção do min, é menor ou igual do que o best deste node max (desta chamada de max_value))
            // entao não vale a pena continuar pois este ramo nunca vai ser escolhido, (pois o min vai escolher o beta que era a sua melhor opção anterior)
            if best >= beta {
                return best;
            }

            if best > alpha {
                alpha = best;
            }
        }

        best
    }

    fn min_value(
        &self,
        depth: i32,
        board: Board,
        start: Instant,
        eng_move_time: i32,
        node_count: &mut u64,
        alpha: i32,
        beta: i32,
        board_history: Vec<u64>,
        ply: i32,
        stop_flag: Arc<AtomicBool>,
        seldepth: &mut i32,
    ) -> i32 {
        *node_count += 1;

        let legal_moves = self.get_legal_moves(&board, Color::Black); // child nodes/positions

        if ply > *seldepth {
            *seldepth = ply;
        }

        if legal_moves.is_empty() {
            // if there are no legal moves now, it is checkmate or stalemate
            if self.is_king_in_check(&board, &Color::Black) {
                // check if black king was checkmated
                // the ply is to penalize longer mates from the root, and prefer shorter ones
                return 1_000_000 - ply; // White wins
            } else {
                return 0; // stalemate draw
            }
        } else if Self::is_three_fold_draw(board.zobrist_hash, &board_history) {
            return 0;
        } else if depth == 0 {
            if self.has_captures(&board, &legal_moves) {
                //quiescence_search: dont stop searching until all captures are resolved
                return self.quiescence_search_black(
                    board,
                    node_count,
                    alpha,
                    beta,
                    ply,
                    seldepth,
                    start,
                    eng_move_time,
                    stop_flag,
                );
            }
            return self.eval(&board);
        }

        if eng_move_time > 0 {
            if start.elapsed().as_millis() > eng_move_time as u128
                || stop_flag.load(Ordering::Relaxed)
            {
                return self.eval(&board);
            }
        } else if eng_move_time == 0 {
            // go infinite
            if stop_flag.load(Ordering::Relaxed) {
                return self.eval(&board);
            }
        }

        let mut best = i32::MAX;
        let mut beta = beta;

        let mut board_history = board_history;

        for m in &legal_moves {
            let mut board_clone = board.clone();

            self.simulate_move(m, &mut board_clone, &Color::Black); // do a move/go down into a branch

            board_history.push(board_clone.zobrist_hash);

            let v = self.max_value(
                depth - 1,
                board_clone.clone(),
                start,
                eng_move_time,
                node_count,
                alpha,
                beta,
                board_history.clone(),
                ply + 1,
                Arc::clone(&stop_flag),
                seldepth,
            );

            if v < best {
                // for each move that we do, we return the one that is the most valuable for black
                best = v;
            }

            board_history.pop();

            // se a melhor opção deste nó min (desta chamada de min_value na arvore de pesquisa), é menor do que o alpha,
            // entao este ramo nunca vai ser escolhido pois o best deste nó seria sempre menor do que a melhor opção do max acima (o alpha)
            if best <= alpha {
                return best;
            }

            if best < beta {
                beta = best;
            }
        }

        best
    }

    fn get_best_move(
        &self,
        depth: i32,
        board: Board,
        maximizing_player: Color,
        eng_move_time: i32,
        start: Instant,
        prev_bm: Move,
        stop_flag: Arc<AtomicBool>,
        seldepth: &mut i32,
    ) -> (Move, i32, bool, u64) {
        let mut best_eval = match self.color {
            Color::Black => i32::MAX,
            Color::White => i32::MIN,
        };
        *seldepth = 1; // reset after every iterative deepening call

        let mut legal_moves = self.get_legal_moves(&board, maximizing_player);

        self.order_moves(&mut legal_moves, prev_bm);

        let mut best_move = *legal_moves
            .get(0)
            .expect("stalemate or checkmate: no legal moves");

        let mut is_full_depth = true; // check if all the nodes up to this depth were searched, or if it stoped mid search
        // let mut total_nodes: u64 = legal_moves.len() as u64;
        let mut total_nodes: u64 = 0;

        let mut alpha = i32::MIN;
        let mut beta = i32::MAX;

        let mut board_hist_clone = self.board_history.clone();

        let ply = 1; // number of moves from the root, used to prefer shorter forced mates

        for m in &legal_moves {
            if eng_move_time > 0 {
                if start.elapsed().as_millis() > eng_move_time as u128
                    || stop_flag.load(Ordering::Relaxed)
                {
                    is_full_depth = false; // there was still another legal move branch to consider, and the search was cancelled because of time
                    break;
                }
            } else if eng_move_time == 0 {
                // go infinite
                if stop_flag.load(Ordering::Relaxed) {
                    is_full_depth = false; // there was still another legal move branch to consider, and the search was cancelled because of time
                    break;
                }
            }

            let mut board_clone = board.clone();

            self.simulate_move(m, &mut board_clone, &maximizing_player);

            board_hist_clone.push(board_clone.zobrist_hash);

            let eval = if Self::is_three_fold_draw(board_clone.zobrist_hash, &board_hist_clone) {
                0
            } else if maximizing_player == Color::White {
                // minimax eval for each of the moves, max calls min and min calls max
                self.min_value(
                    depth - 1,
                    board_clone.clone(),
                    start,
                    eng_move_time,
                    &mut total_nodes,
                    alpha,
                    beta,
                    board_hist_clone.clone(),
                    ply,
                    Arc::clone(&stop_flag),
                    seldepth,
                )
            } else {
                // black just played, now its white to simulate a move
                self.max_value(
                    depth - 1,
                    board_clone.clone(),
                    start,
                    eng_move_time,
                    &mut total_nodes,
                    alpha,
                    beta,
                    board_hist_clone.clone(),
                    ply,
                    Arc::clone(&stop_flag),
                    seldepth,
                )
            };

            board_hist_clone.pop();

            match maximizing_player {
                Color::White => {
                    if eval > best_eval {
                        best_eval = eval;
                        best_move = *m;
                        if best_eval > alpha {
                            alpha = best_eval;
                        };
                    }
                }
                Color::Black => {
                    if eval < best_eval {
                        best_eval = eval;
                        best_move = *m;
                        if best_eval < beta {
                            beta = best_eval;
                        };
                    };
                }
            };
        }

        (best_move, best_eval, is_full_depth, total_nodes)
    }

    fn search_aux(
        &self,
        depth: &mut i32,
        stop_flag: Arc<AtomicBool>,
        eng_move_time: i32,
        start: Instant,
        b_m: &mut Option<Move>,
        info_printed: &mut bool,
        seldepth: &mut i32,
    ) {
        let mut eval: i32 = -1;
        let mut is_full_depth: bool;

        let search_start = std::time::Instant::now();
        let mut duration_ms;
        let mut nodes: u64 = 0;

        match self.get_best_move(
            *depth,
            self.board.clone(),
            self.color,
            eng_move_time,
            start,
            b_m.unwrap(),
            Arc::clone(&stop_flag),
            seldepth,
        ) {
            (m, e, i_f_d, n) => {
                duration_ms = search_start.elapsed().as_millis();

                is_full_depth = i_f_d;
                if is_full_depth {
                    // if it was not stoped mid search
                    nodes = n;
                    *b_m = Some(m);
                    eval = e;
                    *info_printed = true;
                }
            }
        };

        // #[cfg(feature = "uci_info")]
        if is_full_depth {
            // check if all the nodes up to this depth were searched, or if it stoped mid search
            let mate_eval = eval;

            if self.color == Color::Black {
                // to match UCI and GUIs expectation
                eval = -eval;
            }

            // its safer to have the duration > 0.0 check because on lower depths duration can be close to 0, and float division by 0 can be problematic
            let nps = if duration_ms > 0 {
                (nodes as u128) * 1000 / duration_ms
            } else {
                0
            };

            let bm = b_m.unwrap().to_uci();

            if mate_eval > 999_900 || mate_eval < -999_000 {
                let getting_mated_or_mating = if mate_eval > 0 && self.color == Color::Black {
                    -1
                } else {
                    1
                };

                let plies = if mate_eval > 0 {
                    1_000_000 - mate_eval
                } else {
                    1_000_000 + mate_eval
                }; //plies for the mate found

                // let full_moves = ((*depth / 2) + 1) * getting_mated_or_mating; wrong, because search depth could be 5, but the mate could be already found at depth 2, 2 plies deep
                let full_moves = ((plies / 2) + 1) * getting_mated_or_mating;

                if mate_eval > 999_900 {
                    // white is about to mate
                    if self.color == Color::White {
                        // the engine mating the oponent
                        println!(
                            "info depth {depth} seldepth {seldepth} score mate {full_moves} nodes {nodes} time {duration_ms} nps {nps} pv {bm}"
                        );
                    } else {
                        //the engine is getting mated
                        println!(
                            "info depth {depth} seldepth {seldepth} score mate {full_moves} nodes {nodes} time {duration_ms} nps {nps} pv {bm}"
                        );
                    }
                } else if mate_eval < -999_900 {
                    if self.color == Color::White {
                        // the engine is getting mated
                        println!(
                            "info depth {depth} seldepth {seldepth} score mate {full_moves} nodes {nodes} time {duration_ms} nps {nps} pv {bm}"
                        );
                    } else {
                        // the engine mating the oponent
                        println!(
                            "info depth {depth} seldepth {seldepth} score mate {full_moves} nodes {nodes} time {duration_ms} nps {nps} pv {bm}"
                        );
                    }
                }
            } else {
                println!(
                    "info depth {depth} seldepth {seldepth} score cp {eval} nodes {nodes} time {duration_ms} nps {nps} pv {bm}"
                );
            }
        };

        *depth += 1; // iterative deepening
    }
    // returns the best move and updates the move counts on the boards
    pub fn search(
        &mut self,
        moves: Vec<String>,
        times: Option<PlayerTimes>,
        stop_flag: Arc<AtomicBool>,
    ) -> String {
        let mut eng_move_time: i32 = 0;

        match times {
            None => {} // search infinitely until the stop flag
            Some(ref t) => {
                eng_move_time = self.calculate_time(t);
            }
        }

        let start = std::time::Instant::now();

        let mut depth = 1;

        let mut seldepth = 1;

        let mut best_move = String::new();
        let mut b_m: Option<Move> = None;

        let legal_moves = self.get_legal_moves(&self.board, self.color);

        if legal_moves.len() == 1 {
            // so that it does not do iterative deepening on 1 available move
            b_m = legal_moves.first().copied();

            let mut board_clone = self.board.clone();
            self.simulate_move(&b_m.unwrap(), &mut board_clone, &self.color);
            let mut eval = if self.color == Color::Black {
                -self.eval(&board_clone)
            } else {
                self.eval(&board_clone)
            };

            let bm = b_m.unwrap().to_uci();

            // to remove warning in fastchess, we always gotta print some info before saying "bestmove ..."
            // IMPORTANT: the UCI info printed bellow about eval and checkmates is not accurate, and that is completly intentional
            // because the engine does not waste time checking if it will be checkmate, since it can only play 1 move anyways
            // for accurate checkmate and eval info, it would have to a pointless iterative deepening search since it is forced to always play the same move
            if eval > 999_900 {
                // white is about to mate in the next move, which is the only available one
                if self.color == Color::White {
                    // the engine is about to mate the oponent
                    println!("info depth 1 seldepth 1 score mate 1 nodes 1 time 0 nps 0 pv {bm}");
                } else {
                    //the engine is getting mated
                    println!("info depth 1 seldepth 1 score mate -1 nodes 1 time 0 nps 0 pv {bm}");
                }
            } else if eval < -999_900 {
                if self.color == Color::White {
                    // the engine is getting mated
                    println!("info depth 1 seldepth 1 score mate -1 nodes 1 time 0 nps 0 pv {bm}");
                } else {
                    // the engine is about to mate the oponent
                    println!("info depth 1 seldepth 1 score mate 1 nodes 1 time 0 nps 0 pv {bm}");
                }
            } else {
                // this is the branch that should always execute, since the engine does not check for checkmates if it only has 1 move available to play
                println!("info depth 1 seldepth 1 score cp {eval} nodes 1 time 0 nps 0 pv {bm}");
            }
        } else if legal_moves.len() == 0 {
            println!("panicccccccc, no legal moves in the search");
            panic!("no legal moves");
        } else {
            b_m = legal_moves.first().copied(); // so that it has any move and is not None if the time is super low, and the games do not stall in fastchess
            let mut info_printed = false; // to remove warnings in fastchess: before saying bestmove, the engine is expected to always print a line with eval info, so even if is_full_depth is false for depth 1, info must be printed.

            // let mut board_clone = self.board.clone();// to not clone it more than once between diferent depths of iterative deepening

            if times.is_some() {
                //always has to search at least depth 1 to print info to the guis, otherwise we get a warning from fastchess
                while (start.elapsed().as_millis() < eng_move_time as u128
                    && !stop_flag.load(Ordering::Relaxed))
                    || depth == 1
                {
                    self.search_aux(
                        &mut depth,
                        Arc::clone(&stop_flag),
                        eng_move_time,
                        start,
                        &mut b_m,
                        &mut info_printed,
                        &mut seldepth,
                    )
                }
            } else {
                // go infinite
                while !stop_flag.load(Ordering::Relaxed) || depth == 1 {
                    self.search_aux(
                        &mut depth,
                        Arc::clone(&stop_flag),
                        eng_move_time,
                        start,
                        &mut b_m,
                        &mut info_printed,
                        &mut seldepth,
                    )
                }
            }

            let mut b_clone = self.board.clone();

            if !info_printed {
                self.simulate_move(&b_m.unwrap(), &mut b_clone, &self.color);
                let eval = if self.color == Color::Black {
                    -self.eval(&b_clone)
                } else {
                    self.eval(&b_clone)
                };
                let bm = b_m.unwrap().to_uci();
                println!("info depth 1 seldepth 1 score cp {eval} nodes 1 time 0 nps 0 pv {bm}");
            };
        }

        best_move = b_m.unwrap().to_uci();

        let is_capture_or_pawn_move = self.is_capture_or_pawn_move(b_m.unwrap());

        self.board
            .update_move_counts(self.color, is_capture_or_pawn_move);

        self.move_piece(&b_m.unwrap()); // apply engine move to the board

        // self.add_fen(&best_move);
        // self.board_history.push(self.board.zobrist_hash);

        best_move
    }

    // orders best moves to be looked at first to improve alpha beta pruning
    fn order_moves(&self, legal_moves: &mut Vec<Move>, prev_bm: Move) {
        if let Some(pos) = legal_moves.iter().position(|x| *x == prev_bm) {
            legal_moves.swap(0, pos);
        } else {
            println!("PANNIC, previous best move should still be a legal move");
            // panic!("previous best move should still be a legal move")
        }
    }

    fn quiescence_search_black(
        &self,
        board: Board,
        node_count: &mut u64,
        alpha: i32,
        beta: i32,
        ply: i32,
        seldepth: &mut i32,
        start: Instant,
        eng_move_time: i32,
        stop_flag: Arc<AtomicBool>,
    ) -> i32 {
        *node_count += 1;

        if eng_move_time > 0 {
            if start.elapsed().as_millis() > eng_move_time as u128
                || stop_flag.load(Ordering::Relaxed)
            {
                return self.eval(&board);
            }
        } else if eng_move_time == 0 {
            // go infinite
            if stop_flag.load(Ordering::Relaxed) {
                return self.eval(&board);
            }
        }

        if ply > *seldepth {
            *seldepth = ply;
        }

        let stand_pat = self.eval(&board); // means "keep the everything as is": it is very important in case making the capture is worse than doing nothing, since we are only considering captures in the search 
        // Example: our a queen captures a pawn, but then the opponent recaptures with a rook.
        // we would have been better off not capturing at all

        // the oponents best choice (alpha), is never going to choose this branch
        // because stand_pat is only going to get smaller, and its already not good for them
        if stand_pat <= alpha {
            return stand_pat;
        }

        let captures = self.generate_captures(&board, Color::Black);

        if captures.is_empty() {
            // no captures left, return eval
            return self.eval(&board);
        }

        let mut best = stand_pat; //our best option is doing nothing, since we are only considering captures in this search

        let mut beta = beta.min(stand_pat); // we are only going to chose captures that are better than doing nothing

        for c in &captures {
            let mut board_clone = board.clone();

            self.simulate_move(c, &mut board_clone, &Color::Black); // do a move/go down into a branch

            let v = self.quiescence_search_white(
                board_clone.clone(),
                node_count,
                alpha,
                beta,
                ply + 1,
                seldepth,
                start,
                eng_move_time,
                Arc::clone(&stop_flag),
            );

            if v < best {
                // for each move that we do, we return the one that is the most valuable for black
                best = v;
            }

            // se a melhor opção deste nó min (desta chamada de min_value na arvore de pesquisa), é menor do que o alpha,
            // entao este ramo nunca vai ser escolhido pois o best deste nó seria sempre menor do que a melhor opção do max acima (o alpha)
            if best <= alpha {
                return best;
            }

            if best < beta {
                beta = best;
            }
        }

        best
    }

    fn quiescence_search_white(
        &self,
        board: Board,
        node_count: &mut u64,
        alpha: i32,
        beta: i32,
        ply: i32,
        seldepth: &mut i32,
        start: Instant,
        eng_move_time: i32,
        stop_flag: Arc<AtomicBool>,
    ) -> i32 {
        *node_count += 1;

        if eng_move_time > 0 {
            if start.elapsed().as_millis() > eng_move_time as u128
                || stop_flag.load(Ordering::Relaxed)
            {
                return self.eval(&board);
            }
        } else if eng_move_time == 0 {
            // go infinite
            if stop_flag.load(Ordering::Relaxed) {
                return self.eval(&board);
            }
        }

        if ply > *seldepth {
            *seldepth = ply;
        }

        let stand_pat = self.eval(&board); // means "keep the everything as is": it is very important in case making the capture is worse than doing nothing, since we are only considering captures from now on 
        // Example: our a queen captures a pawn, but then the opponent recaptures with a rook.
        // we would have been better off not capturing at all

        // the oponents best choice (beta), is never going to choose this branch
        // because stand_pat is only going to get bigger, and its already not good for them
        if stand_pat >= beta {
            return stand_pat;
        }

        let captures = self.generate_captures(&board, Color::White);

        if captures.is_empty() {
            // no captures left, return eval
            return self.eval(&board);
        }

        let mut best = stand_pat; //our best option is doing nothing, since we are only considering captures in this search

        let mut alpha = alpha.max(stand_pat); // we are only going to chose captures that are better than doing nothing

        for c in &captures {
            let mut board_clone = board.clone();

            self.simulate_move(c, &mut board_clone, &Color::White); // do a move/go down into a branch

            let v = self.quiescence_search_black(
                board_clone.clone(),
                node_count,
                alpha,
                beta,
                ply + 1,
                seldepth,
                start,
                eng_move_time,
                Arc::clone(&stop_flag),
            );

            if v > best {
                // for capture that we do, we return the one that is the most valuable for white
                best = v;
            }

            // se o beta (melhor opção do min, é menor ou igual do que o best deste node max (desta chamada de max_value))
            // entao não vale a pena continuar pois este ramo nunca vai ser escolhido, (pois o min vai escolher o beta que era a sua melhor opção anterior)
            if best >= beta {
                return best;
            }

            if best > alpha {
                alpha = best;
            }
        }

        best
    }

    fn generate_captures(&self, board: &Board, color: Color) -> Vec<Move> {
        let mut captures = self.get_legal_moves(board, color);

        for c in &captures.clone() {
            if !self.is_capture(board, c) {
                captures.retain(|x| x != c);
            }
        }

        captures
    }

    fn has_captures(&self, board: &Board, legal_moves: &Vec<Move>) -> bool {
        for m in legal_moves {
            if self.is_capture(board, m) {
                return true;
            }
        }

        false
    }

    fn is_capture(&self, board: &Board, m: &Move) -> bool {
        let final_square_piece = board.get_piece_at_square(&m.final_square());

        match final_square_piece {
            None => false,
            Some(_) => true,
        }
    }

    fn is_three_fold_draw(hash: u64, board_history: &Vec<u64>) -> bool {
        let mut repetition_count = 0;
        let is_draw = false;

        for pos in board_history.iter().rev() {
            // search backwards to enconter a draw faster
            if *pos == hash {
                repetition_count += 1;
            }
            if repetition_count >= 2 {
                return true;
            }
        }

        is_draw
    }

    fn is_capture_or_pawn_move(&self, m: Move) -> bool {
        // checks important info for the 50 move draw rule
        let moving_piece = self
            .board
            .get_piece_at_square(&m.starting_square())
            .expect("moving from an empty square");

        if moving_piece.piece == Piece::Pawn {
            return true;
        }

        let final_square = self.board.get_piece_at_square(&m.final_square());

        match final_square {
            None => false,
            Some(p) => {
                if p.color != self.current_player {
                    true
                } else {
                    println!("PANNIC, cant capture friendly pieces");
                    panic!("cant capture friendly pieces");
                }
            }
        }
    }

    fn eval(&self, board: &Board) -> i32 {
        let mut eval = 0;
        for (i, row) in board.get_pieces().iter().enumerate() {
            for (j, p) in row.iter().enumerate() {
                let is_white = true;
                match p {
                    None => continue,
                    Some(piece) => {
                        if piece.piece == Piece::Pawn && board.get_en_passant().is_some() {
                            // to not count the ep ghos t
                            let ep_square = board.get_en_passant().unwrap();

                            if ep_square.rank == (i as i8) + 1
                                && board::file_to_num(ep_square.file) == j as u8
                            {
                                continue;
                            }
                        }
                        eval += piece.value(i, j);

                        // if (j == 3 || j == 4) && (i == 3 || i == 4) { // bonus for being in the 4 central squares
                        //     match piece.color {
                        //         Color::White => eval += 50,
                        //         Color::Black => eval -= 50,
                        //     }
                        // }
                    }
                }
            }
        }

        eval
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

//conditional compilation
// the entire test module is not compiled unless the command is cargo test
mod tests;
