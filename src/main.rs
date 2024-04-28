mod lexer;
mod error;
mod parser;

use rustyline::{error::ReadlineError, history::DefaultHistory, Config, EditMode, Editor};
use termion::color;

use crate::{lexer::Lexer, parser::Parser};

fn main() {
    let mut stdin = Editor::<(), DefaultHistory>::with_config(
        Config::builder()
            .edit_mode(EditMode::Vi)
            .build()
    ).unwrap();

    println!();

    loop {
        match stdin.readline(">> ") {
            Ok(input) => {
                let mut lexer = Lexer::new(&input);
                match lexer.tokenize() {
                    Ok(tokens) => {
                        let mut parser = Parser::new(tokens);
                        match parser.parse() {
                            Ok(ast) => {
                                println!("{}", ast);
                            },
                            Err(err) => {
                                err.print(&input);
                            }
                        }
                    },
                    Err(err) => {
                        err.print(&input);
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("^D");
                break;
            },
            Err(err) => {
                println!("{}stdin error{}: {}", color::Fg(color::Red), err, color::Fg(color::Reset));
                break;
            }
        }
    }
}
