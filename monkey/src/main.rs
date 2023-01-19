#![feature(assert_matches)]
#![feature(let_chains)]
#![feature(is_some_and)]

pub mod lexer;
pub mod repl;
pub mod token;

fn main() {
    let user = whoami::username();
    println!("Hello {user}, this is the Monkey programming language");
    println!("Feel free to type in commands");
    repl::start();
}
