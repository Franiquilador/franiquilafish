use std::{fmt::DebugStruct, i32, mem::transmute, vec};
use std::io::{stdout, Write};

use crate::chess::board;
use board::Board;
use crate::chess::move_square::Move;
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
    color_playing: Color,
    board: Board,
    legal_moves: Vec<Move>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            is_running: true,
            game_state: GameState::Created,
            color: Color::Black,
            color_playing: Color::White,
            board: Board::new(),
            legal_moves: vec![],
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn close(&mut self) {
        self.is_running = false;
    }

    fn update_active_player(&mut self) {
        self.board.set_current_player_in_fen(&self.color_playing);
        if let Color::Black = self.color_playing {
            self.color_playing = Color::White;
        } else {
            self.color_playing = Color::Black;
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
        &self.color_playing
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
        self.legal_moves = self.board.get_legal_moves(&self.color_playing);

        
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

    fn simulate_move(&mut self, m: &Move, board: &mut Board) {
        let moving_piece = board.get_piece_at_square(&m.get_starting_square());

        board.update_square(moving_piece, &m.final_square());

        board.update_square(None, &&m.get_starting_square());
    }

    pub fn apply_moves(&mut self, moves: Vec<String>) {
        self.board = Board::new();

        for move_before in &moves {
            let m = Move::from_uci(move_before).unwrap();
            self.move_piece(&m);
        } 
    }

    pub fn search(&mut self, moves: Vec<String>, times: PlayerTimes, stop_flag: Arc<AtomicBool>) -> String {
        // self.apply_moves(moves);

        let legal_moves = self.board.get_legal_moves(&self.color);

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

        for m in legal_moves {
            let mut board_clone = self.board.clone();
            self.simulate_move(&m, &mut board_clone);

            let e = self.eval(&board_clone);

            if self.color == Color::White {
                if e > best_eval {
                    best_eval = e;
                    best_move = m.to_uci();
                }
            } else {
                if e < best_eval {
                    best_eval = e;
                    best_move = m.to_uci();
                }
            }
        }

        // println!("das3");
        // stdout().flush().unwrap();

        // "f7f6".to_string()
        best_move
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