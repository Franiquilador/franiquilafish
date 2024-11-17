use std::io::{self, Write};
use chess_cl::chess::{game::Game, start_match, piece::{Piece::*, Piece, Color::*, Color}};

enum Command {
    Help,
    Quit,
    NewGame,
    Unknown(String),
}

impl Command {
    // reads input as &str and returns an enum for pattern matching
    fn from_input(input: &str) -> Self {
        match input.trim().to_lowercase().as_str() {
            "help" => Command::Help,
            "quit" => Command::Quit,
            "new" => Command::NewGame,
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


// time, serde and board ranks (1-8) and files (a-h) to std in
fn main() {    
    let mut game = Game::new();
    println!("Type 'help' for commands");

    while game.is_running() {
        let mut input = String::new();
        if !game.has_started() {
            print!("> ");
        }
        
        /*
        * so that "> " is printed immediatly instead of waiting for the output buffer to full,
        * to then system call the os to write to stdout
        */
        std::io::stdout().flush().expect("failed to flush the output buffer");
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let command = Command::from_input(&input);
        process_command(command, &mut game);
    }
    
}

fn process_command(command: Command, game: &mut Game) {
    match command {
        Command::Help => process_help(),
        Command::Quit => game.close(),
        Command::NewGame => process_new(game),
        Command::Unknown(cmd) => {
            println!("Unknow command '{cmd}'. Type 'help' for commands");
        }
    }
}

fn process_help() {
    println!("help - prints usefull information about all available commands");
    println!("new - creates a new chess game");
    println!("quit - terminate the chess game");
}

fn process_new(game: &mut Game) {
    game.start();
    print_board(game.get_board().get_pieces(), game.get_active_player());
    
}

fn print_board(pieces: &[[Piece; 8]; 8], active_player: &Color) {
    println!("  +---+---+---+---+---+---+---+---+");
    for (i, rank) in pieces.iter().enumerate() {
        print!("{} |", 8 - i);
        for piece in rank {
            match piece {
                Empty => print!("   |"),
                Rook(White) => print!(" R |"),
                Knight(White) => print!(" N |"),
                Bishop(White) => print!(" B |"),
                Queen(White) => print!(" Q |"),
                King(White) => print!(" K |"),
                Pawn(White) => print!(" P |"),
                Rook(Black) => print!(" r |"),
                Knight(Black) => print!(" n |"),
                Bishop(Black) => print!(" b |"),
                Queen(Black) => print!(" q |"),
                King(Black) => print!(" k |"),
                Pawn(Black) => print!(" p |"),
            }
        }
        println!();
        println!("  +---+---+---+---+---+---+---+---+");
    }
    println!("    a   b   c   d   e   f   g   h");
    println!();
    match active_player {
        White => print!("White to play: "),
        Black => print!("Black to play: "),
    }

}