use franiquilafish::chess::engine::PlayerTimes;
use franiquilafish::chess::{
    engine::{Color, Engine},
    piece::{ChessPiece, Piece as P},
};

use std::io::stdout;
use std::{
    io::{Write, stdin},
    sync::mpsc::{Receiver, Sender},
};

use std::sync::mpsc;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::thread;

// time, serde
fn main() {
    // let mut engine = Engine::new();

    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();

    let trimed = input.trim(); // to remove the '\n' in the end for processing it

    match trimed {
        "uci" => uci(),
        _ => {
            // println!("Unknown command, type {HELP} for help"),
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

fn uci() {
    id_outputs();

    println!("uciok"); // after initializing parameters;
    stdout().flush().unwrap();

    // Shared stop flag (atomic = thread-safe, no locks needed)
    let stop = Arc::new(AtomicBool::new(false));

    let stop_clone = Arc::clone(&stop);

    let (producer, consumer) = mpsc::channel();

    //search thread, the main threads blocks waiting for GUI commands on the stdin
    // the search thrd ocasionally checks for msgs from main to see if the engine told it to stop the search
    let _search_thread = thread::spawn(move || {
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
                // let _ = producer.send("isready".to_string());
                println!("readyok");
                stdout().flush().unwrap();
            }
            "ucinewgame" => {
                let _ = producer.send("ucinewgame".to_string());
            }
            "stop" => {
                stop.store(true, Ordering::Relaxed);
            } //atomicbool operation guarantees the flag is updated by only one thread at a time, since the flag is shared between threads
            "quit" => break,
            line => {
                let _ = producer.send(line.to_string());
            }
        }
    }
}

fn search_thread(stop_clone: Arc<AtomicBool>, consumer: Receiver<String>) {
    let mut engine = Engine::new();

    let mut moves: Vec<String> = vec![];

    loop {
        let cmd: String = consumer.recv().unwrap(); // blocks waiting for input from the GUI, received in the main thread

        let times: PlayerTimes;

        match cmd.as_str() {
            "isready" => {
                // this is never reached now, we respond instantly in the main thread like we are supposed to
                println!("readyok"); // after initializing engine parameters chosed by the GUI
                stdout().flush().unwrap();
            }
            "ucinewgame" => {
                moves.clear();
                engine.start();
                engine.clear();
            }
            line => {
                let parts: Vec<_> = line.split_whitespace().collect();

                if parts.is_empty() {
                    continue;
                }

                let mut pairs = parts.windows(2);

                match parts[0] {
                    "perft" => {
                        let max_depth = parts[1].parse().expect("error parsing str");
                        engine.perft(max_depth);
                    }
                    "wp" => {
                        //webperft
                        let depth = parts[1].parse().expect("error parsing str");
                        engine.web_perft(depth);
                    }
                    "position" => {
                        let fen_or_startpos_opt = parts.iter().find(|s| **s == "fen");

                        // if its not fen it must be startpos
                        let is_fen = fen_or_startpos_opt.is_some();

                        if is_fen {
                            let fen_parts = parts
                                .iter()
                                .skip_while(|s| **s != "fen")
                                .skip(1) // skip "fen" itself
                                .take_while(|s| **s != "moves")
                                .cloned()
                                .collect::<Vec<_>>();
                            engine.load_from_fen(fen_parts);
                        } // do not need an else because the default Engine::new() already builds a board from the startpos

                        moves = parts
                            .iter()
                            .skip_while(|s| s != &&"moves")
                            .skip(1) // skip moves word itself
                            .map(|s| s.to_string())
                            .collect();

                        // println!("stored moves {:?}", moves);
                        // stdout().flush().unwrap();

                        engine.apply_moves(moves.clone(), is_fen); // apply oponents moves
                    }
                    "go" => {
                        stop_clone.store(false, Ordering::Relaxed); // reset the stop command in a new search

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

                        let movetime = parts
                            .windows(2) // has to have a new iterator like this, instead of reusing pairs
                            .find(|window| window[0] == "movetime")
                            .and_then(|window| window[1].parse::<i32>().ok())
                            .unwrap_or(0);

                        times = PlayerTimes {
                            wtime,
                            btime,
                            winc,
                            binc,
                            movetime,
                        };
                        let mut time = None; // we assume go infinite was sent until proven otherwise
                        if (btime != 0 && wtime != 0) || movetime != 0 {
                            time = Some(times);
                        }

                        engine.set_color(*engine.get_active_player());

                        let best_move = engine.search(time, Arc::clone(&stop_clone));

                        println!("bestmove {best_move}");
                        stdout().flush().unwrap();
                    }
                    _ => { /*panic!("empty first string");*/ } //  unreachable
                }
            }
        }
    }
}

fn id_outputs() {
    println!("id name Diesel");
    stdout().flush().unwrap();
    println!("id author Francisco Figueiredo");
    stdout().flush().unwrap();
}

#[allow(clippy::clone_on_copy)]
fn _print_board(pieces: [[Option<ChessPiece>; 8]; 8], active_player: &Color) {
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
                    },
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
