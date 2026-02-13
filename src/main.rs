use chess_cl::chess::{
    engine::{Color, Engine, GameState}, move_square::Move, piece::{ChessPiece as CP, ChessPiece, Piece as P}, start_match
};
use std::io::{self, Write, stdin};

use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex, atomic::AtomicBool};

enum Command {
    Help,
    Uci,
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
            HELP => Self::Help,
            QUIT => Self::Quit,
            NEW_GAME => Self::NewGame,
            "uci" => Self::Uci,
            other => Self::Unknown(other.to_string()),
        }
    }

    fn get_move_from_uci(input: &str) -> Option<Move> {
        Move::from_uci_coords(input)
    }
}

enum UciCommand { // all possible comands from the GUI using the UCI protocol
    IsReady,
    UciNewGame,
    Quit,
}

impl UciCommand {
    fn from(cmd: &str) -> Option<Self> {
        match cmd {
            "isready" => Some(Self::IsReady),
            "quit" => Some(Self::Quit),
            _ => {
                // println!("Uci command from the GUI not supported");
                None
            }
        }
    }
}

// time, serde
fn main() {
    let mut engine = Engine::new();
    // println!("This is command line chess!");
    // println!("Type '{HELP}' for commands");

    
    while engine.is_running() {
        let mut input = String::new();
        if !engine.has_started() {
            // print!("> ");
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

        match engine.game_state() {
            GameState::Playing => {
                let m = Command::get_move_from_uci(&input);
                match m {
                    Some(valid) => process_move(&mut engine, valid),
                    None => println!("invalid move"),
                }
            },
            _ => {
                let command = Command::from_input(&input);
                process_command(command, &mut engine);
            },
        }
    }
    
/*
    let mut a = [[1, 4], [2, 0]];

    a.reverse();

    println!("{:?}", a);
    
    
    let t = 'a'..='h';
    for c in t {
        print!("{}", c);
    }*/

}

fn process_move(engine: &mut Engine, m: Move) {
    if engine.is_legal(&m) {
        engine.move_piece(&m);
    } else {
        println!("ilegal move");
        println!();
    }
    // dbg!(m);
    print_board(engine.get_board().get_pieces(), engine.get_active_player());
    // todo!("process move");
}

fn process_command(command: Command, engine: &mut Engine) {
    match command {
            Command::Help => process_help(),
            Command::Uci => uci(),
            Command::Quit => engine.close(),
            Command::NewGame => process_new(engine),
            Command::Unknown(cmd) => {
                println!("Unknow command '{cmd}'. Type '{HELP}' for commands");
            }
    }
}

fn uci() {
    id_outputs();

    // Shared stop flag (atomic = thread-safe, no locks needed)
    let stop = Arc::new(AtomicBool::new(false));

    let (producer, consumer) = mpsc::channel();

    let stop_clone = stop.clone();

    //search thread, the main threads blocks waiting for GUI commands on the stdin
    // the search thrd ocasionally checks for msgs from main to see if the engine told it to stop the search
    let search_thread = thread::spawn(move || { 
        loop {
            let cmd: String = consumer.recv().unwrap(); // blocks waiting for input from the GUI, received in the main thread
            match cmd.as_str() {
                "isready" => {
                    println!("readyok"); // after initializing engine parameters chosed by the GUI
                },
                "ucinewgame" => {

                },
                "go" => {

                },
                _ => {
                    println!("unknown uci command sent by the main thread");
                },
            }

        }
    
    });

    println!("uciok"); // after initializing parameters;

    let mut cmd = String::new();

    stdin().read_line(&mut cmd).unwrap(); //blocks waiting for a command from the GUI, or for a '\n' enter

    let cmd = cmd.trim();

    // let uci_cmd = UciCommand::from(cmd).expect("Uci command not supported by the engine");

    loop {

        match cmd {
            "isready" => {
                let _ = producer.send("isready".to_string());
            },
            "ucinewgame" => {
                let _ = producer.send("ucinewgame".to_string());
            },
            "quit" => break,
            _ => {
                // println!("unknown uci command")
                continue;
            },
        }
    }

    let res = producer.send("uci".to_string());
}

fn id_outputs() {
    println!("id name Diesel");
    println!("id name Francisco Figueiredo");
}

fn process_help() {
    println!("{HELP} - prints usefull information about all available commands");
    println!("{NEW_GAME} - creates a new chess game");
    println!("uci - enters the uci protocol with a GUI");
    println!("{QUIT} - terminate the chess game");
}

fn process_new(game: &mut Engine) {
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
