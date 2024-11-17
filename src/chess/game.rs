use crate::chess::board;
use board::Board;
use crate::chess::piece::{Color};

use super::piece::Piece;

//#[derive(Clone)]
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
            game_state: GameState::Playing,
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
        if let Color::White = self.color_playing {
            self.color_playing = Color::Black;
            self.board.set_current_player_in_fen(Color::Black);
        } else {
            self.color_playing = Color::White;
            self.board.set_current_player_in_fen(Color::White);
        }
    }

    pub fn start(&self) {
        todo!()
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }
}





enum GameState {
    Playing,
    CheckMate,
}

