use std::io;
use std::io::*;


pub fn run_file(file_path: &str) {
    println!("Hello, World!");
}

pub fn run_prompt() {
    let handle = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        //io::stdin().read_line(&mut input).expect("Jotain meni pieleen");
        handle.read_line(&mut input).expect("Jotain meni pieleen");
    }
}
