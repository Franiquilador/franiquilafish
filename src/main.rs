use chess_cl::chess::{
    game::{Color, Game, GameState},
    piece::{ChessPiece as CP, ChessPiece, Piece as P},
    start_match,
};
use std::io::{self, Write};

enum Command {
    Help,
    Quit,
    NewGame,
    Unknown(String),
}

const HELP: &str = "help";
const NEW_GAME: &str = "new";
const QUIT: &str = "quit";


impl Command {
    // reads input as &str and returns an enum for pattern matching
    fn from_input(input: &str) -> Self {
        match input.trim().to_lowercase().as_str() {
            HELP => Command::Help,
            QUIT => Command::Quit,
            NEW_GAME => Command::NewGame,
            other => Command::Unknown(other.to_string()),
        }
    }
}

/*
fn unsafe_crash() {
    let adress = 0x125usize;
    let r1 = adress as *const i32;

    println!("r1 is {}", unsafe { *r1 } );
}
*/

// time, serde
fn main() {
    let mut game = Game::new();
    println!("This is chess in the command line!");
    println!("Type '{HELP}' for commands");

    while game.is_running() {
        let mut input = String::new();
        if !game.has_started() {
            print!("> ");
        }

        /*
         * so that "> " is printed immediatly instead of waiting for the output buffer to full,
         * to then system call the os to write to stdout
         */
        std::io::stdout()
            .flush()
            .expect("failed to flush the output buffer");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let command = Command::from_input(&input);
        process_command(command, &mut game);
    }
}

fn process_command(command: Command, game: &mut Game) {
    match game.game_state() {
        GameState::Playing => todo!("move commands"),
        _ => match command {
            Command::Help => process_help(),
            Command::Quit => game.close(),
            Command::NewGame => process_new(game),
            Command::Unknown(cmd) => {
                println!("Unknow command '{cmd}'. Type 'help' for commands");
            }
        },
    }
}

fn process_help() {
    println!("{HELP} - prints usefull information about all available commands");
    println!("{NEW_GAME} - creates a new chess game");
    println!("{QUIT} - terminate the chess game");
}

fn process_new(game: &mut Game) {
    game.start();
    print_board(game.get_board().get_pieces(), game.get_active_player());
}

fn print_board(pieces: &[[Option<ChessPiece>; 8]; 8], active_player: &Color) {
    println!("  +---+---+---+---+---+---+---+---+");

    for (i, rank) in pieces.iter().enumerate() {
        print!("{} |", 8 - i);
        for piece in rank {
            match piece {
                None => print!("   |"),
                Some(p) => match p {
                    CP::White(P::Rook) => print!(" R |"),
                    CP::White(P::Knight) => print!(" N |"),
                    CP::White(P::Bishop) => print!(" B |"),
                    CP::White(P::Queen) => print!(" Q |"),
                    CP::White(P::King) => print!(" K |"),
                    CP::White(P::Pawn) => print!(" P |"),
                    CP::Black(P::Rook) => print!(" r |"),
                    CP::Black(P::Knight) => print!(" n |"),
                    CP::Black(P::Bishop) => print!(" b |"),
                    CP::Black(P::Queen) => print!(" q |"),
                    CP::Black(P::King) => print!(" k |"),
                    CP::Black(P::Pawn) => print!(" p |"),
                },
            }
        }
        println!();
        println!("  +---+---+---+---+---+---+---+---+");
    }
    println!("    a   b   c   d   e   f   g   h");
    println!();
    match active_player {
        Color::White => print!("White to play: "),
        Color::Black => print!("Black to play: "),
    }
}
