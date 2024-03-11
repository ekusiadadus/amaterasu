use std::{env, error::Error, fs, process::exit};

fn run(source: &str) -> Result<(), Box<dyn Error>> {
    return Err("Not implemented".into());
}

fn run_file(path: &str) -> Result<(), Box<dyn Error>> {
    match fs::read_to_string(path) {
        Ok(contents) => {
            return run(&contents);
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            exit(74);
        }
    }
}

fn run_prompt() -> Result<(), Box<dyn Error>> {
    loop {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => match run(&input) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error running input: {}", e);
                    exit(74);
                }
            },
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                exit(74);
            }
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: jlox [script]");
        exit(64);
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error running file: {}", e);
                exit(74);
            }
        }
    } else {
        match run_prompt() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error running prompt: {}", e);
                exit(74);
            }
        }
    }
    dbg!(args);
}
