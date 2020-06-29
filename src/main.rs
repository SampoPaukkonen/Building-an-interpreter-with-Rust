use std::env;
use std::process;
use jlox;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            jlox::run_prompt();
            process::exit(0);
        },
        2 => {
            jlox::run_file(&args[1]);
        },
        _ => {
            println!("Usage: jlox [script]");
            process::exit(0);
        },
    }
}
