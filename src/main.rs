use std::{env, error::Error, fs, process::exit};

fn run_file(path: &str) -> Result<(), Box<dyn Error>> {
    let source = fs::read_to_string(path)?;

    // run(source);
    Ok(())
}

fn run_prompt() {
    // loop {
    //     let mut input = String::new();
    //     std::io::stdin().read_line(&mut input).unwrap();
    //     run(input);
    // }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: jlox [script]");
        exit(64);
    } else if args.len() == 2 {
        run_file(&args[2]);
    } else {
        run_prompt();
    }
    dbg!(args);
}
