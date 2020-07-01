use std::io;
use std::io::*;
use std::fs;
use std::fmt;

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Lox {
        Lox { had_error: false }
    }

    pub fn run_file(&self, file_path: &str) {
        let source: String = fs::read_to_string(file_path).expect("Unable to read given file");
        Lox::run(&source);
    }
    pub fn run_prompt(&self) {
        let handle = io::stdin();
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            handle.read_line(&mut input).expect("Jotain meni pieleen");
            Lox::run(&input);
        }
    }

    fn run(source: &str) {
        println!("{}", source);
    }

    pub fn error (&mut self, line: u32, message: &str) {
        Lox::report(self, line, "", message);
    }

    fn report(&mut self, line: u32, location: &str, message: &str) {
        eprintln!("[line {}] Error {}: {}",line, location, message);
        self.had_error = true;
    }
}



pub struct Scanner<'a> {
 pub source: String<'a>,
 source_as_bytes: std::str::Bytes<'a>,
 pub tokens: Vec<Token>,
 start: u32,
 current: u32,
 line: u32,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            source_as_bytes: source.bytes(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 0,
        }
    }

    //pub fn scan_tokens(&self) {
    //    while (self.current < self.source.len()) {
    //        self.start = self.current;
    //        self.scan_token();
    //    }
    //}

    pub fn scan_tokens(&mut self) {
        for (i, c) in self.source.bytes().enumerate() {
            let new_token = match c {
                //'(' => self.add_token(TokenType::LEFT_PAREN);
                b')' => TokenType::RIGHT_PAREN,
                b'{' => TokenType::LEFT_BRACE,
                b'}' => TokenType::RIGHT_BRACE,
                b',' => TokenType::COMMA,
                b'.' => TokenType::DOT,
                b'+' => TokenType::PLUS,
                b'-' => TokenType::MINUS,
                b';' => TokenType::SEMICOLON,
                b'*' => TokenType::STAR,
                _ => TokenType::CLASS,
            };
            //self.tokens.push(Token::new(new_token, ))

        }
        self.tokens.push(Token::new(TokenType::EOF, String::from(""), String::from("FOOBAR"), line));
        //tokens
    }

    fn add_token(&mut self, new_token: TokenType) {
        let text: String = String::from(self.source.by[self.start..self.current]);
    }

}

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: u32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: String, line: u32) -> Token {
        Token { token_type, lexeme, literal, line }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {}, {}, {})", self.token_type, self.lexeme, self.literal, self.line)
    }
}

#[derive(Debug)]
pub enum TokenType {
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,
    IDENTIFIER,
    STRING,
    NUMBER,
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
    EOF,
}
