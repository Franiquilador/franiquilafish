use crate::chess::board;
use board::Board;
// use crate::chess::piece::{Color};

pub enum Color {
    Black,
    White,
}

pub enum GameState {
    Playing, // created and started (ongoing game)
    CheckMate(Color),
    StaleMate(Color),
    Created, // new game created but not started
}


pub struct Game {
    is_running: bool,
    game_state: GameState,
    color_playing: Color,
    board: Board,
}

impl Game {
    pub fn new() -> Self {
        Game {
            is_running: true,
            game_state: GameState::Created,
            color_playing: Color::White,
            board: Board::new(),
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn close(&mut self) {
        self.is_running = false;
    }

    pub fn play(&mut self) {
        if let Color::Black = self.color_playing {
            self.color_playing = Color::Black;
            self.board.set_current_player_in_fen(Color::Black);
        } else {
            self.color_playing = Color::Black;
            self.board.set_current_player_in_fen(Color::Black);
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
            GameState::StaleMate(c) => true,
        }
    }
}

