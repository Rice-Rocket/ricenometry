mod lexer;
mod error;
mod parser;
mod strategies;

use rustyline::{error::ReadlineError, history::DefaultHistory, Config, EditMode, Editor};
use termion::color;

use crate::{lexer::Lexer, strategies::{print_runstrats, select_runstrats, RunStrategies}, parser::Parser};

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
                print_runstrats();
                let Ok(opts) = select_runstrats(&mut stdin) else { break };
                match opts {
                    RunStrategies::ShowAST => {
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
                    RunStrategies::Tokenize => {
                        let mut lexer = Lexer::new(&input);
                        match lexer.tokenize() {
                            Ok(tokens) => {
                                println!("{:?}", tokens);
                            },
                            Err(err) => {
                                err.print(&input);
                            }
                        }
                    },
                    other => {
                        let mut lexer = Lexer::new(&input);
                        match lexer.tokenize() {
                            Ok(tokens) => {
                                let mut parser = Parser::new(tokens);
                                match parser.parse() {
                                    Ok(ast) => {
                                        match other {
                                            _ => unreachable!(),
                                        }
                                    },
                                    Err(err) => {
                                        err.print(&input)
                                    }
                                }
                            },
                            Err(err) => {
                                err.print(&input);
                            }
                        }
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
