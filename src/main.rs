mod lexer;
mod error;

use rustyline::{error::ReadlineError, history::DefaultHistory, Config, EditMode, Editor};
use termion::color;

use crate::lexer::Lexer;

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
                        println!("{:?}", tokens)
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
