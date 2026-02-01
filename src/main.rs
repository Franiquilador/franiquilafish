use chess_cl::chess::{
    game::{Color, Game, GameState}, move_square::{Move}, piece::{ChessPiece as CP, ChessPiece, Piece as P}, start_match
};
use std::io::{self, Write};

enum Command {
    Help,
    Quit,
    NewGame,
    Unknown(String),
}

const HELP: &str = "h";
const NEW_GAME: &str = "n";
const QUIT: &str = "q";


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

    fn get_move_from_uci(input: &str) -> Option<Move> {
        Move::from_uci_coords(input)
    }
}

// time, serde
fn main() {
    let mut game = Game::new();
    println!("This is command line chess!");
    println!("Type '{HELP}' for commands");

    
    while game.is_running() {
        let mut input = String::new();
        if !game.has_started() {
            print!("> ");
        }

        /*
         * so that "> " is printed immediatly instead of waiting for the output buffer to be full,
         * to then system call the os to write to stdout
         */
        std::io::stdout()
            .flush()
            .expect("failed to flush the output buffer");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match game.game_state() {
            GameState::Playing => {
                let m = Command::get_move_from_uci(&input);
                match m {
                    Some(valid) => process_move(&mut game, valid),
                    None => println!("invalid move"),
                }
            },
            _ => {
                let command = Command::from_input(&input);
                process_command(command, &mut game);
            },
        }
    }
    

    // let mut a = [[1, 4], [2, 0]];

    // a.reverse();

    // println!("{:?}", a);
    
    // let t = 'a'..='h';
    // for c in t {
    //     print!("{}", c);
    // }
}

fn process_move(game: &mut Game, m: Move) {
    if game.is_legal(&m) {
        game.move_piece(&m);
    } else {
        println!("ilegal move");
        println!();
    }
    // dbg!(m);
    print_board(game.get_board().get_pieces(), game.get_active_player());
    // todo!("process move");
}

fn process_command(command: Command, game: &mut Game) {
    match command {
            Command::Help => process_help(),
            Command::Quit => game.close(),
            Command::NewGame => process_new(game),
            Command::Unknown(cmd) => {
                println!("Unknow command '{cmd}'. Type '{HELP}' for commands");
            }
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

fn print_board(mut pieces: [[Option<ChessPiece>; 8]; 8], active_player: &Color) {
    println!("  +---+---+---+---+---+---+---+---+");

    let mut cloned_pieces = pieces.clone();
    cloned_pieces.reverse();

    // for row in pieces.iter_mut() {
    //     row.reverse();
    // }

    for (i, rank) in cloned_pieces.iter().enumerate() {
        print!("{} |", 8 - i);
        for piece in rank {
            match piece {
                None => print!("   |"),
                Some(p) => match p.color {
                    Color::White => match p.piece {
                        P::Rook => print!(" R |"),
                        P::Knight => print!(" N |"),
                        P::Bishop => print!(" B |"),
                        P::Queen => print!(" Q |"),
                        P::King => print!(" K |"),
                        P::Pawn => print!(" P |"),
                    },

                    Color::Black => match p.piece {
                        P::Rook => print!(" r |"),
                        P::Knight => print!(" n |"),
                        P::Bishop => print!(" b |"),
                        P::Queen => print!(" q |"),
                        P::King => print!(" k |"),
                        P::Pawn => print!(" p |"),    
                    }
                },
            }
        }
        println!();
        println!("  +---+---+---+---+---+---+---+---+");
    }
    
    println!("    a   b   c   d   e   f   g   h");
    
    println!();
    println!("Capital letters are white");
    match active_player {
        Color::White => print!("White to play: "),
        Color::Black => print!("Black to play: "),
    }
    
}
