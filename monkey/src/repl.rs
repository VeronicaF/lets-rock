use crate::lexer::Lexer;
use std::io;
use std::io::{Read, Write};

const PROMPT: &[u8; 3] = b">> ";

pub fn start() -> ! {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        let mut input = String::new();
        stdout.write_all(PROMPT).unwrap();
        stdout.flush().unwrap();
        stdin.read_line(&mut input).unwrap();
        let mut lexer = Lexer::new(input);

        while let Some(token) = lexer.next_token() {
            stdout
                .write_all(format!("{token:#?}\n").as_bytes())
                .unwrap();
            stdout.flush().unwrap();
        }
    }
}
