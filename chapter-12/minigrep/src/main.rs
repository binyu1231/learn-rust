use std::env;
use std::process;

use minigrep::Config;

// cargo run test poem.txt

fn main() {
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    
    // let config = Config::new(&args).unwrap_or_else(|err| {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // 输出到标准错误
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // println!("Searching for \"{}\" in file: {}", config.query, config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }

    // cmd
    // $ set CASE_INSENSITIVE=1
    // cargo run to poem.txt

    // bash
    // (CASE_INSENSITIVE=1; cargo run to poem.txt)

    // 
    // cargo run to poem.txt > output.txt
}



