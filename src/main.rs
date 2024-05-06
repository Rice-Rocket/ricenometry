#![allow(unused)]

mod lexer;
mod error;
mod parser;
mod strategies;
mod expr;
mod prelude;
mod utils;

use expr::Expr;
use lexer::token::Token;
use parser::node::Node;
use rustyline::{error::ReadlineError, history::DefaultHistory, Config, EditMode, Editor};
use termion::color;

use crate::{lexer::Lexer, strategies::{print_runstrats, select_runstrats, RunStrategies}, parser::Parser};

fn tokenize(input: &str) -> Option<Vec<Token>> {
    let mut lexer = Lexer::new(input);
    match lexer.tokenize() {
        Ok(tokens) => {
            Some(tokens)
        },
        Err(err) => {
            err.print(input);
            None
        }
    }
}

fn parse(input: &str, tokens: &[Token]) -> Option<Node> {
    let mut parser = Parser::new(tokens.to_vec());
    match parser.parse() {
        Ok(ast) => {
            Some(ast)
        },
        Err(err) => {
            err.print(input);
            None
        }
    }
}

fn to_expr(input: &str, ast: Node) -> Option<Expr> {
    match Expr::convert(ast) {
        Ok(expr) => {
            Some(expr)
        },
        Err(err) => {
            err.print(input);
            None
        }
    }
}

fn run(input: &str, opts: RunStrategies) {
    let Some(tokens) = tokenize(input) else { return };
    let Some(ast) = parse(input, &tokens) else { return };
    let Some(expr) = to_expr(input, ast.clone()) else { return };
    match opts {
        RunStrategies::Tokenize => {
            println!();
            println!("{:?}", tokens);
            println!();
        },
        RunStrategies::ShowAST => {
            println!();
            println!("{}", ast);
        },
        RunStrategies::Simplify => {
            println!();
            println!("{}", expr.simplify());
            println!();
        }
    }
}

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
                let Ok(opts) = select_runstrats(&mut stdin, 0) else { break };
                run(&input, opts);
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
