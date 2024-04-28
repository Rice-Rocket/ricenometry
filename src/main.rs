mod lexer;

use rustyline::{DefaultEditor, error::ReadlineError};
use termion::color;

fn main() {
    let mut stdin = DefaultEditor::new().unwrap();

    println!();

    loop {
        match stdin.readline(">> ") {
            Ok(input) => {
                println!("{}", input);
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
