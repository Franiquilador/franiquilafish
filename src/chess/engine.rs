use std::clone;
use std::{fmt::DebugStruct, i32, mem::transmute, vec};
use std::io::{stdout, Write};

use crate::chess::board;
use crate::chess::piece::{ChessPiece, Piece};
use board::Board;
use crate::chess::move_square::{Move, Square};
use std::sync::{Arc, Mutex, atomic::AtomicBool};
use crate::chess::move_square::Promotion;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

pub enum GameState {
    Playing, // created and started (ongoing game)
    CheckMate(Color),
    StaleMate,
    Created, // new game created but not started
}

pub struct PlayerTimes { // in miliseconds
    pub wtime: i32,
    pub btime: i32,
    pub winc: i32,
    pub binc: i32,
}


pub struct Engine {
    is_running: bool,
    game_state: GameState,
    color: Color,
    current_player: Color,
    board: Board,
    legal_moves: Vec<Move>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            is_running: true,
            game_state: GameState::Created,
            color: Color::Black,
            current_player: Color::White,
            board: Board::new(),
            legal_moves: vec![],
        }
    }

    pub fn load_from_fen(&mut self, fen_parts: Vec<&str>) { // is called every round if the game started from fen
        self.board = Board::from_fen(fen_parts);
        self.current_player = self.board.current_player;
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn close(&mut self) {
        self.is_running = false;
    }

    fn update_active_player(&mut self) {
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

    fn promote_piece(moving_piece: &mut Option<ChessPiece>, m: &Move, board: &mut Board, current_player: Color) {// promotes a piece if it is a pawn and is promoting in the move
        match m.promotion {
            None => {
                *moving_piece = board.get_piece_at_square(&m.starting_square()); // none if the square is empty
            }
            Some(promotion) => {
                *moving_piece = Some(ChessPiece { color: current_player,
                piece: match promotion {
                        Promotion::Bishop => Piece::Bishop,
                        Promotion::Knight => Piece::Knight,
                        Promotion::Rook => Piece::Rook,
                        Promotion::Queen => Piece::Queen,
                } })
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
        Self::update_castling_rights(&mut self.board, moving_piece, final_piece, &starting_square, &final_square);
        
        // dbg!(&self.board);
        self.update_active_player();
        // todo!();
    }

    fn simulate_move(&self, m: &Move, board: &mut Board) {
        let mut moving_piece= None;

        let starting_square = m.starting_square();
        let final_square = m.final_square();

        Self::promote_piece(&mut moving_piece, m, board, board.current_player);

        let final_piece = board.get_piece_at_square(&final_square);

        let is_a_castle = board.is_a_castle(*m);

        board.update_square(moving_piece, &final_square);

        board.update_square(None, &starting_square);

        if is_a_castle {
            Self::move_castled_rook(board, &final_square);
        }

        Self::update_en_passant(m, board, moving_piece, final_piece);
        Self::update_castling_rights(board, moving_piece, final_piece, &starting_square, &final_square);

    }

    fn move_castled_rook(board: &mut Board, final_square: &Square) { // m is the kings castling move notation
        // let is_king_side = if engine_final_square == Square::new('g', 8).unwrap() { true } else { false };
        // let is_queen_side = if engine_final_square == Square::new('c', 8).unwrap() { true } else { false };
        if final_square.rank == 1 {// white
            if final_square.file == 'g' { // kingside, rook moves from h1 to f1
                board.update_square(Some(ChessPiece { piece: Piece::Rook, color: Color::White }), &Square { rank: 1, file: 'f' });
                board.update_square(None, &Square { rank: 1, file: 'h' });
            } else if final_square.file == 'c' { // queenside, rook moves from a1 to d1
                board.update_square(Some(ChessPiece { piece: Piece::Rook, color: Color::White }), &Square { rank: 1, file: 'd' });
                board.update_square(None, &Square { rank: 1, file: 'a' });
            } else {
                println!("PANNICCC, should be a castling move");
                panic!("should be a castling move");
            }
        } else if final_square.rank == 8 { // black
            if final_square.file == 'g' { // kingside, rook moves from h8 to f8
                board.update_square(Some(ChessPiece { piece: Piece::Rook, color: Color::Black }), &Square { rank: 8, file: 'f' });
                board.update_square(None, &Square { rank: 8, file: 'h' });
            } else if final_square.file == 'c' { // queenside, rook moves from a8 to d8
                board.update_square(Some(ChessPiece { piece: Piece::Rook, color: Color::Black }), &Square { rank: 8, file: 'd' });
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

    fn update_castling_rights(board: &mut Board, moving_piece: Option<ChessPiece>, final_piece_before_move: Option<ChessPiece>
    ,starting_square: &Square, final_square: &Square) {
        match moving_piece {
            Some(p) => {
                match p.piece {
                    Piece::King => { // king moves
                        board.remove_all_castling(&p.color);
                    }
                    Piece::Rook => { // friendly rook moves
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
            Some(p) => { // if a friendly rook is captured on its home square, no castling can happen on that side
                if p.piece == Piece::Rook {
                    match p.color { // the color of the rook being captured
                        Color::Black => {
                            if final_square.rank == 8 && (final_square.file == 'a' || final_square.file == 'h') {
                                board.remove_castling(&Color::Black, final_square);
                            }
                        }
                        Color::White => {
                            if final_square.rank == 1 && (final_square.file == 'a' || final_square.file == 'h') {
                                board.remove_castling(&Color::White, final_square);
                            }
                        }
                    }
                }
            }
        }
    }

    fn update_en_passant(m: &Move, board: &mut Board, moving_piece: Option<ChessPiece>, final_piece_before_move: Option<ChessPiece>) { // static method to avoid borrowing problems       
        let starting_square = m.starting_square();
        let final_square = m.final_square();
        
        let rank_dif = (final_square.rank - starting_square.rank).abs();
        let file_dif = (final_square.file as i8 - starting_square.file as i8).abs();
    
        let is_moving_piece_pawn = moving_piece.map(|p| p.piece == Piece::Pawn).unwrap_or(false);
        let is_double_push_white= is_moving_piece_pawn && starting_square.rank == 2 && rank_dif == 2;
        let is_double_push_black = is_moving_piece_pawn && starting_square.rank == 7 && rank_dif == 2;
        let is_en_passant = is_double_push_white || is_double_push_black;

        let is_white_capture = starting_square.rank == 5; // white capture black en passant

        // let final_piece = board.get_piece_at_square(&m.final_square());

        match final_piece_before_move { // to check if it is a capture, to both pawns (ghost target and the actual pawn)
            None => {}
            Some(p) => {
                let e_p = board.get_en_passant();

                match e_p {
                    None => {}
                    Some(s) => {

                        //its en passant
                        if final_square == s && (moving_piece.unwrap().piece == Piece::Pawn) && file_dif == 1 { // we also have to remove the original pawn that moved 2 squares
                            board.update_square(None, &Square::new(s.file, if is_white_capture { s.rank - 1 } else { s.rank + 1} ).unwrap());
                        }
                    }
                }
            }
        };

        if is_double_push_white { // update the target square
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
            self.board = Board::new();// not in case of fen, because load_from_fen() is called from main before this method.
            self.current_player = self.board.current_player;
        }

        for move_before in &moves {
            let m = Move::from_uci(move_before).unwrap();
            self.move_piece(&m); // updates board and active player color aswell
        }
    }

    fn get_legal_moves(&self, board: &Board, color: Color) -> Vec<Move> {
        let mut pseudo_legal_moves = board.pseudo_legal_moves(&color);
        self.remove_checks(&mut pseudo_legal_moves, color, board);
        pseudo_legal_moves // become legal after removing moves that leave the king in check
    }

    fn pawn_attacks(board: &Board, color: &Color) -> Vec<Square> { // needed because of tricky castle and pawn captures interactions in remove_checks()
        let mut attacked = vec![];
        for (i, row) in board.get_pieces().iter().enumerate() {
            for (j, piece) in row.iter().enumerate() {
                if let Some(p) = piece {
                    if p.color == *color && p.piece == Piece::Pawn {
                        let file = board::num_to_file(j as u8); // j is the file index (0..7) -> (a..=h)
                        let rank = (i as i32) + 1;        // i is the rank index (0..7 → 1..8)

                        let pos = Square::new(file, rank).unwrap();

                        match color {
                            Color::Black => {
                                match pos.offset(1, -1) { // returns None if the pos is outside the board
                                    None => {}
                                    Some(s) => { attacked.push(s); }
                                };
                                match pos.offset(-1, -1) { // returns None if the pos is outside the board
                                    None => {}
                                    Some(s) => { attacked.push(s); }
                                };
                            }
                            Color::White => {
                                match pos.offset(1, 1) { // returns None if the pos is outside the board
                                    None => {}
                                    Some(s) => { attacked.push(s); }
                                };
                                match pos.offset(-1, 1) { // returns None if the pos is outside the board
                                    None => {}
                                    Some(s) => { attacked.push(s); }
                                };
                            }
                        }
                    }
                }
            }
        }
        attacked
    }

    fn remove_checks(&self, legal_moves: &mut Vec<Move>, color: Color, board: &Board) { // removes moves that make the engines king remain in check
        for m in legal_moves.clone() { // for each of the engine legal moves, check if it does not leave the engines king in check
            let mut board_clone = board.clone();
            let is_a_castle = board_clone.is_a_castle(m);

            self.simulate_move(&m, &mut board_clone);

            let other_player = if color == Color::Black { Color::White } else { Color::Black };
            let other_player_legal_moves = board_clone.pseudo_legal_moves(&other_player);

            // eprintln!("Simulated black move: {}", m.to_uci());
            // eprintln!("White pseudo-legal moves after: {:?}", 
                // other_player_legal_moves.iter().map(|x| x.to_uci()).collect::<Vec<_>>());
            
            for other_m in other_player_legal_moves {
                let final_square= other_m.final_square();
                if is_a_castle { 
                    let engine_final_square = m.final_square();
                    match color {
                        Color::Black => {
                            let is_king_side = if engine_final_square == Square::new('g', 8).unwrap() { true } else { false };
                            let is_queen_side = if engine_final_square == Square::new('c', 8).unwrap() { true } else { false };

                            let pawn_attacks = Self::pawn_attacks(&board_clone, &other_player); // because board_clone.pseudo_legal_moves(&other_player); does not generate pawn checks, since simulate move already castled the king
                            if pawn_attacks.contains(&Square::new('e', 8).unwrap()) {
                                legal_moves.retain(|mov| mov != &m);
                            }

                            if final_square == Square::new('e', 8).unwrap() { // the king cant castle to escape a check
                                legal_moves.retain(|mov| mov != &m);
                            }

                            if final_square == Square::new('f', 8).unwrap() && is_king_side {// check for the kings path when castling
                                legal_moves.retain(|mov| mov != &m);
                            } else if final_square == Square::new('d', 8).unwrap() && is_queen_side {
                                legal_moves.retain(|mov| mov != &m);
                            }
                        }
                        Color::White => {
                            let is_king_side = if engine_final_square == Square::new('g', 1).unwrap() { true } else { false };
                            let is_queen_side = if engine_final_square == Square::new('c', 1).unwrap() { true } else { false };

                            let pawn_attacks = Self::pawn_attacks(&board_clone, &other_player); // because board_clone.pseudo_legal_moves(&other_player); does not generate pawn checks, since simulate move already castled the king
                            if pawn_attacks.contains(&Square::new('e', 1).unwrap()) {
                                legal_moves.retain(|mov| mov != &m);
                            }

                            if final_square == Square::new('e', 1).unwrap() { // the king cant castle to escape a check
                                legal_moves.retain(|mov| mov != &m);
                            }

                            if final_square == Square::new('f', 1).unwrap() && is_king_side {// check for the kings path when castling
                                legal_moves.retain(|mov| mov != &m);
                            } else if final_square == Square::new('d', 1).unwrap() && is_queen_side {
                                legal_moves.retain(|mov| mov != &m);
                            }
                        }
                    };
                }

                let final_square_piece = board_clone.get_piece_at_square(&final_square);

                match final_square_piece {
                    None => {} // move into an empty Square
                    Some(other) => {
                        if other.piece == Piece::King {// engines king can be captured the next move (he will be in check, so the move is ilegal)
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

        for d in 1..=depth { // iterative deepening
            if d != depth {// to remove iterative deepening
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
            println!("{possible_positions} possible positions at depth {d} generated in {:.3} seconds, ({:.0} nodes per second)", duration.as_secs_f64(), nodes_per_second);
        }
        println!("------------------- finished webperft ---------------------");
    }

    fn perft_aux_wp(&self, depth: i32, board: &mut Board, color: Color) -> i32 { //max depth this iteration
        if depth == 0 {
            return 1;
        }
        
        let mut nodes = 0;
        let next_color = if color == Color::White { Color::Black } else { Color::White };

        for m in self.get_legal_moves(board, color) {
            let mut board_for_this_branch = board.clone();
            
            self.simulate_move(&m, &mut board_for_this_branch);

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

        for depth in 1..=max_depth { // iterative deepening

            let start = std::time::Instant::now();

            //
            let possible_positions = self.perft_aux(depth, &mut board_clone, self.current_player);
            let duration = start.elapsed(); // not acurate because perft_aux does iterative deepening, and it does not account for time of previous

            // moves generated per_second
            let nodes_per_second = possible_positions as f64 / duration.as_secs_f64();
            
            println!("{possible_positions} possible positions at depth {depth} generated in {:.3} seconds, ({:.0} nodes per second)", duration.as_secs_f64(), nodes_per_second);
        }
        println!("------------------- finished perft -------------------");
    }

    fn unit_perft(&self, depth: i32) -> i32 {
        println!("starting unit perft at depth {depth}...");
        let mut board_clone = self.board.clone();

        let start = std::time::Instant::now();

        let possible_positions = self.perft_aux(depth, &mut board_clone, self.current_player);
        let duration = start.elapsed(); // not acurate because perft_aux does iterative deepening, and it does not account for time of previous

        // moves generated per_second
        let nodes_per_second = possible_positions as f64 / duration.as_secs_f64();
            
        println!("unit: {possible_positions} possible positions at depth {depth} generated in {:.3} seconds, ({:.0} nodes per second)", duration.as_secs_f64(), nodes_per_second);
        
        println!("------------------- finished perft -------------------");
        
        possible_positions
    }

    fn perft_aux(&self, depth: i32, board: &mut Board, color: Color) -> i32 { //max depth this iteration
        if depth == 0 {
            return 1;
        }
        
        let mut nodes = 0;

        for m in self.get_legal_moves(board, color) {
            let mut board_for_this_branch = board.clone();

            self.simulate_move(&m, &mut board_for_this_branch);

            nodes += self.perft_aux(depth - 1, &mut board_for_this_branch,
            if color == Color::White { Color::Black } else { Color::White });
        }
        
        return nodes;
    }

    // returns the best move and updates the move counts on the boards
    pub fn search(&mut self, moves: Vec<String>, times: PlayerTimes, stop_flag: Arc<AtomicBool>) -> String {
        // self.apply_moves(moves);

        let board_clone = self.board.clone();

        let legal_moves = self.get_legal_moves(&board_clone, self.color); // get all legal moves from the current board
        // println!("das1");
        // stdout().flush().unwrap();
        
        let mut best_eval = match self.color {
            Color::Black => i32::MAX,
            Color::White => i32::MIN,
        };
        let mut best_move = String::new();
        // println!("das3");
        // stdout().flush().unwrap();

        if !legal_moves.is_empty() {
            best_move = legal_moves[0].to_uci();
        } else {
            println!("PANICCCCCCCCCCCCCCCCCCC, draw? no legal moves that the engine knows");
            stdout().flush().unwrap();
            // panic!("no legal moves, maybe draw?");
        }
        // println!("das4");
        // stdout().flush().unwrap();

        // println!("{:?}", legal_moves.clone());
        // stdout().flush().unwrap();

        // println!("das2");
        // stdout().flush().unwrap();
        let mut b_m = legal_moves.get(0).expect("no legal moves");

        for m in legal_moves.iter() {
            let mut board_clone = self.board.clone();
            self.simulate_move(&m, &mut board_clone);

            let e = self.eval(&board_clone);

            if self.color == Color::White {
                if e > best_eval {
                    best_eval = e;
                    b_m = m;
                    best_move = m.to_uci();
                }
            } else {
                if e < best_eval {
                    best_eval = e;
                    best_move = m.to_uci();
                }
            }
        }

        let is_capture_or_pawn_move = self.is_capture_or_pawn_move(*b_m);

        self.board.update_move_counts(self.color, is_capture_or_pawn_move);

        // println!("das3");
        // stdout().flush().unwrap();

        // "f7f6".to_string()
        self.move_piece(b_m); // apply engine move to the board
        best_move
    }

    fn is_capture_or_pawn_move(&self, m: Move) -> bool { // checks important info for the 50 move draw rule
        let moving_piece = self.board.get_piece_at_square(&m.starting_square()).expect("moving from an empty square");

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
                    panic!("cant capture friendly pieces");
                }
            }
        }
    }

    fn eval(&self, board: &Board) -> i32 {
        // println!("das6");
        // stdout().flush().unwrap();
        let mut eval = 0;
        for row in board.get_pieces() {
            for p in row {
                let is_white = true;
                match p {
                    None => continue,
                    Some(piece) => eval += piece.value(),
                }
            }
        }

        // println!("das7");
        // stdout().flush().unwrap();
        eval
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}