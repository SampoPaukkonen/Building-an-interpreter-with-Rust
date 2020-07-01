use std::env;
use std::process;
use jlox;

fn main() {
    let args: Vec<String> = env::args().collect();
    let lox = jlox::Lox::new();
    let tmp = jlox::Token::new(jlox::TokenType::AND, String::from("Moikka"), String::from("kaikille!"), 10);
    println!("TÄMÄ ON TESTI: {}", tmp);
    match args.len() {
        1 => {
            lox.run_prompt();
            process::exit(0);
        },
        2 => {
            lox.run_file(&args[1]);
        }
        _ => {
            println!("Usage: jlox [script]");
            process::exit(0);
        },
    }
}
