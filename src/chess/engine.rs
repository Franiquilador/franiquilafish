use std::vec;

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
    color_playing: Color,
    board: Board,
    legal_moves: Vec<Move>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            is_running: true,
            game_state: GameState::Created,
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

        dbg!(&self.legal_moves);
        println!("num of legal moves: {}", self.legal_moves.len());
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
        self.board.update_square(moving_piece, &m.get_final_square());

        // dbg!(&self.board);
        self.board.update_square(None, &&m.get_starting_square());
        
        // dbg!(&self.board);
        self.update_active_player();
        // todo!();

    }

    pub fn search(&mut self, moves: Vec<&str>, times: PlayerTimes, stop_flag: Arc<AtomicBool>) -> String {

        "f7f6".to_string()
    }
}