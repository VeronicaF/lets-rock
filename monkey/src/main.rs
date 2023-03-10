#![feature(assert_matches)]
#![feature(let_chains)]
#![feature(is_some_and)]

mod ast;
pub mod lexer;
pub mod repl;
pub mod token;
mod parser;

fn main() {
    let user = whoami::username();
    println!("Hello {user}, this is the Monkey programming language");
    println!("Feel free to type in commands");
    repl::start();
}
