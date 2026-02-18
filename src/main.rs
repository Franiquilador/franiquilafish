use diesel::chess::{
    engine::{Color, Engine, GameState}, move_square::Move, piece::{ChessPiece as CP, ChessPiece, Piece as P}, start_match
};
use diesel::chess::engine::PlayerTimes;

use std::{io::stdout, os::windows};
use std::{io::{self, Write, stdin}, sync::mpsc::{Receiver, Sender}};

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
        Move::from_uci(input)
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
    // let mut engine = Engine::new();
    /* 
    println!("This is command line chess!");
    println!("Type '{HELP}' for commands");

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
    */
    
    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();

    let trimed = input.trim(); // to remove the '\n' in the end for processing it

    match trimed {
        "uci" => uci(),
        _ => {
            // println!("Unknown command, type {HELP} for help"),

        },
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
                // println!("Unknow command '{cmd}'. Type '{HELP}' for commands");
            },
    }
}

fn uci() {
    id_outputs();

    println!("uciok"); // after initializing parameters;
    stdout().flush().unwrap();

    // Shared stop flag (atomic = thread-safe, no locks needed)
    let mut stop = Arc::new(AtomicBool::new(false));

    let stop_clone = Arc::clone(&stop);

    let (producer, consumer) = mpsc::channel();

    //search thread, the main threads blocks waiting for GUI commands on the stdin
    // the search thrd ocasionally checks for msgs from main to see if the engine told it to stop the search
    let search_thread = thread::spawn(move || {
        search_thread(stop_clone, consumer);
    });

    main_uci_thread(producer, stop);
}

fn main_uci_thread(producer: Sender<String>, stop: Arc<AtomicBool>) {
    // let uci_cmd = UciCommand::from(cmd).expect("Uci command not supported by the engine");

    loop {
        let mut cmd = String::new();

        stdin().read_line(&mut cmd).unwrap(); //blocks waiting for a command from the GUI, or for a '\n' enter

        let cmd = cmd.trim();

        if cmd.is_empty() {
            continue;
        }

        match cmd {
            "isready" => {
                let _ = producer.send("isready".to_string());
            },
            "ucinewgame" => {
                let _ = producer.send("ucinewgame".to_string());
            },
            "stop" => { stop.store(true, std::sync::atomic::Ordering::Relaxed); },  //atomicbool operation guarantees the flag is updated by only one thread at a time, since the flag is shared between threads
            "quit" => break,
            line => {
                producer.send(line.to_string());
            },
        }
    }
}

fn search_thread(stop_clone: Arc<AtomicBool>, consumer: Receiver<String>) {
    let mut engine = Engine::new();

    let mut moves: Vec<String> = vec![];
        
    loop {
        let cmd: String = consumer.recv().unwrap(); // blocks waiting for input from the GUI, received in the main thread
            
        let mut times = PlayerTimes {
            wtime: 0,
            btime: 0,
            winc: 0,
            binc: 0,
        };

        match cmd.as_str() {
            "isready" => {
                println!("readyok"); // after initializing engine parameters chosed by the GUI
                stdout().flush().unwrap();
            },
            "ucinewgame" => {
                moves.clear();
                engine.start();
            },
            line => {
                let parts: Vec<_> = line.split_whitespace().collect();

                if parts.is_empty() {
                    continue;
                }

                let mut pairs = parts.windows(2);
                
                match parts[0] {
                    "position" => {
                        moves = parts.iter()
                            .skip_while(|s| s != &&"moves")
                            .skip(1) // skip moves word itself
                            .map(|s| s.to_string())
                            .collect();

                        // println!("stored moves {:?}", moves);
                        // stdout().flush().unwrap();

                        engine.apply_moves(moves.clone());
                    },
                    "go" => {
                        let wtime = pairs
                        .find(|window| window[0] == "wtime")
                        .and_then(|window| window[1].parse::<i32>().ok())
                        .unwrap_or(0); // 0 if there is no wtime word, instead of panicking

                        let btime = pairs
                        .find(|window| window[0] == "btime")
                        .and_then(|window| window[1].parse::<i32>().ok())
                        .unwrap_or(0);

                        let winc = pairs
                        .find(|window| window[0] == "winc")
                        .and_then(|window| window[1].parse::<i32>().ok())
                        .unwrap_or(0);

                        let binc = pairs
                        .find(|window| window[0] == "binc")
                        .and_then(|window| window[1].parse::<i32>().ok())
                        .unwrap_or(0);

                        times = PlayerTimes {
                                    wtime: wtime,
                                    btime: btime,
                                    winc: winc,
                                    binc: binc,
                                };

                        let color = match moves.len() % 2 == 0 { // engine color
                            true => Color::White,
                            false => Color::Black,
                        };
                        // println!("{:?}", moves);
                        // stdout().flush().unwrap();

                        // println!("{:?}", color);
                        // stdout().flush().unwrap();

                        engine.set_color(color);
                        
                        
                        // println!("das");
                        // stdout().flush().unwrap();

                        let best_move = engine.search(moves.clone(), times, Arc::clone(&stop_clone));
                            
                        print!("bestmove {best_move}\n");
                        stdout().flush().unwrap();
                        },
                        _ => { /*panic!("empty first string");*/ }, //  unreachable
                    }
                },
            }
        }
}


fn id_outputs() {
    println!("id name Diesel");
    stdout().flush().unwrap();
    println!("id author Francisco Figueiredo");
    stdout().flush().unwrap();
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
