use std::io::{self, Write};
use chess_cl::chess::{game::Game, start_match};

enum Command {
    Help,
    Quit,
    Start,
    Unknown(String),
}

impl Command {
    // reads input as &str and returns an enum for pattern matching
    fn from_input(input: &str) -> Self {
        match input.trim().to_lowercase().as_str() {
            "help" => Command::Help,
            "quit" => Command::Quit,
            "start" => Command::Start,
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

fn main() {    
    let mut game = Game::new();
    println!("Type 'help' for commands");

    while game.is_running() {
        let mut input = String::new();
        print!("> ");

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
        Command::Start => process_start(game),
        Command::Unknown(cmd) => {
            println!("Unknow command '{cmd}'. Type 'help' for commands");
        }
    }
}

fn process_help() {
    println!("help - prints usefull information about all available commands");
    println!("start - start a chess game");
    println!("quit - terminate the chess game");
}

fn process_start(game: &mut Game) {
    game.start();
    print_board(game.get_board().get_fen());
    
}

fn print_board(fen: &str) {
    todo!()
}