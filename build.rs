use std::process::Command;

fn main() {

    /*
    this is a cargo build script that is compiled and ran at compile time,
    before everything else in the cargo package

    it is just a rust program at the end of the day
    it is always compiled in Debug mode everytime because this will never be shipped, it is just an auxialiary script to help build the main executable in release potentially 

    This gets the hash/commit version of the project from the last commit and writes it to a new ENV variable
    then main.rs can go to that env variable and get it so that every binary knows which commit from my repo originated it

    this is usefull if I or an end user finds a bug in a binary of franiquilafish 
    and I want to know the commit that generated the binary to correct the bug.

    they might not have git installed on their pc trying to see the version in the final delivered binary would crash
    and even if they had git installed they probably would not even have the franiquilafish repo locally

      */

    let output = Command::new("git")
        .args(["rev-parse", "HEAD"]) // HEAD means get the hash for the commit we are currently on
        .output() // spawns a completly new child process. It is the equivalent to doing fork() + exec() in a Unix system
        //it blocks until the child process finishes and collects all of its output 
        .unwrap();

    let hash = String::from_utf8(output.stdout).unwrap();// collect the raw bytes into a string
    let trimmed = hash.trim(); //remove spaces and \n

    //create a new env variable that will be read in main.rs and inform cargo using println with "cargo:" prefix 
    println!("cargo:rustc-env=GIT_COMMIT={}", trimmed);
}
