use proc_macros::{IterEnum, StringifyEnum};
use rustyline::{history::DefaultHistory, Editor};
use termion::{style, color, cursor};

#[derive(Clone, Copy, IterEnum, StringifyEnum)]
pub enum RunStrategies {
    /// Simplify
    Simplify,
    /// Show AST
    ShowAST,
    /// Tokenize
    Tokenize,
}

pub fn print_runstrats() {
    RunStrategies::iter_fields().enumerate().for_each(|(i, x)| {
        println!("{}{}[{}] {}{}{}", color::Fg(color::Green), style::Bold, i, color::Fg(color::Reset), style::Reset, x.stringify_pretty());
    })
}


pub fn select_runstrats(stdin: &mut Editor<(), DefaultHistory>, depth: u16) -> rustyline::Result<RunStrategies> {
    match stdin.readline("= ") {
        Ok(input) => {
            if let Ok(index) = input.parse::<usize>() {
                if let Some(opt) = RunStrategies::iter_fields().nth(index) {
                    print!("{}{}", cursor::Up(RunStrategies::iter_fields().count() as u16 + 1 + depth), termion::clear::AfterCursor);
                    Ok(opt)
                } else {
                    select_runstrats(stdin, depth + 1)
                }
            } else {
                select_runstrats(stdin, depth + 1)
            }
        },
        Err(err) => {
            Err(err)
        }
    }
}
