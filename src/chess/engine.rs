use std::clone;
use std::{fmt::DebugStruct, i32, mem::transmute, vec};
use std::io::{stdout, Write};

use crate::chess::board;
use crate::chess::piece::Piece;
use board::Board;
use crate::chess::move_square::{Move, Square};
use std::sync::{Arc, Mutex, atomic::AtomicBool};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Color {
    Black,
    White,
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

    //  pre: self.is_legal(m)
    pub fn move_piece(&mut self, m: &Move) {
        
        let moving_piece = self.board.get_piece_at_square(&m.get_starting_square()); // none if the square is empty
        // dbg!(&self.board);
        self.board.update_square(moving_piece, &m.final_square());

        // dbg!(&self.board);
        self.board.update_square(None, &&m.get_starting_square());
        
        // dbg!(&self.board);
        self.update_active_player();
        // todo!();

    }

    fn simulate_move(&self, m: &Move, board: &mut Board) {
        let moving_piece = board.get_piece_at_square(&m.get_starting_square());

        board.update_square(moving_piece, &m.final_square());

        board.update_square(None, &&m.get_starting_square());
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

    fn remove_checks(&self, legal_moves: &mut Vec<Move>, color: Color, board: &Board) { // removes moves that make the engines king remain in check
        for m in legal_moves.clone() { // for each of the engine legal moves, check if it does not leave the engines king in check
            let mut board_clone = board.clone();
            self.simulate_move(&m, &mut board_clone);

            let other_player = if color == Color::Black { Color::White } else { Color::Black };
            let other_player_legal_moves = board_clone.pseudo_legal_moves(&other_player);
            
            for other_m in other_player_legal_moves {
                let final_square= other_m.final_square();
                let final_square_piece = board_clone.get_piece_at_square(&final_square);

                match final_square_piece {
                    None => {} // move into an empty Square
                    Some(other) => {
                        if other.piece == Piece::King {
                            legal_moves.retain(|mov| mov != &m);
                        }
                    }
                }
            }
        }
    }

    pub fn perft(&mut self, max_depth: i32) {
        let mut board_clone = self.board.clone();

        for depth in 1..=max_depth { // iterative deepening

            let start = std::time::Instant::now();

            //
            let possible_positions = self.perft_aux(depth, &mut board_clone, self.current_player);
            let duration = start.elapsed();

            // moves generated per_second
            let nodes_per_second = possible_positions as f64 / duration.as_secs_f64();
            
            println!("{possible_positions} possible positions at depth {depth} generated in {:.3} seconds, ({:.0} nodes per second)", duration.as_secs_f64(), nodes_per_second);
        }
        println!("------------------- finished perft -------------------");
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

        let legal_moves = self.get_legal_moves(&self.board, self.color); // get all legal moves from the current board
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
        let moving_piece = self.board.get_piece_at_square(&m.get_starting_square()).expect("moving from an empty square");

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