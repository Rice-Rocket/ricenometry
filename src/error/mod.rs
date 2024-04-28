pub mod position;
pub mod span;

use span::Span;
use termion::{color, style};


pub enum Error {
    UnknownCharacter(String, Span),
    Syntax(String, Span),
}


#[allow(dead_code)]
impl Error {
    pub fn span(&self) -> Span {
        match self {
            Self::UnknownCharacter(_, span) => *span,
            Self::Syntax(_, span) => *span,
        }
    }

    pub fn with_span(self, span: Span) -> Self {
        match self {
            Self::UnknownCharacter(details, _) => Self::UnknownCharacter(details, span),
            Self::Syntax(details, _) => Self::Syntax(details, span),
        }
    }

    pub fn print(&self, source: &str) {
        print!("\n{}{}error{}: ", color::Fg(color::Red), style::Bold, color::Fg(color::Reset));
        let src_lines: Vec<&str> = source.split('\n').collect();

        match self {
            Self::UnknownCharacter(details, span) => {
                print!("unknown character");
                self.print_details(details, span, src_lines);
            },
            Self::Syntax(details, span) => {
                print!("syntax");
                self.print_details(details, span, src_lines);
            }
        }
    }

    fn print_details(&self, details: &String, span: &Span, src_lines: Vec<&str>) {
        print!(
            "\n {}{}-->{} {}:{}\n{} {}{}|\n",
            color::Fg(color::Blue), style::Bold, style::Reset, 
            span.pos_1.line, span.pos_1.column,
            String::from(" ").repeat(span.pos_1.line.ilog10() as usize + 1),
            color::Fg(color::Blue), style::Bold,
        );

        println!(
            "{}{}{} | {}{}{}",
            color::Fg(color::Blue), style::Bold, span.pos_1.line,
            color::Fg(color::Reset), style::Reset,
            src_lines[span.pos_1.line - 1],
        );

        print!(
            "{}{}{} | {}{}{} {}{}{}\n\n",
            String::from(" ").repeat(span.pos_1.line.ilog10() as usize + 1),
            color::Fg(color::Blue), style::Bold,
            String::from(" ").repeat(span.pos_1.column - 1),
            color::Fg(color::Yellow),
            String::from("^").repeat((span.pos_2.column - span.pos_1.column).max(1)),
            details, color::Fg(color::Reset), style::Reset
        );
    }
}
