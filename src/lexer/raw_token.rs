use proc_macros::IterEnum;
use fancy_regex::Regex;

#[derive(Debug, Clone, Copy, IterEnum)]
pub enum RawTokenType {
    Decimal,
    Identifier,
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    GreaterThan,
    LessThan,
    GreaterThanEq,
    LessThanEq,
    Equals,
    NotEquals,
    Pipe,
    Bang,
    Comma,
    Semicolon,
    Tick,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Whitespace,
    Eol,
    Eof,
}

impl RawTokenType {
    pub fn regex(self) -> Option<Regex> {
        match self {
            RawTokenType::Decimal => Some(Regex::new(r"([0-9]+\.?([0-9]+)?|\.[0-9]+)").unwrap()),
            RawTokenType::Identifier => Some(Regex::new(r"([a-zA-Z_][a-zA-Z0-9_]*)").unwrap()),
            RawTokenType::Add => Some(Regex::new(r"\+").unwrap()),
            RawTokenType::Sub => Some(Regex::new(r"-").unwrap()),
            RawTokenType::Mul => Some(Regex::new(r"\*").unwrap()),
            RawTokenType::Div => Some(Regex::new(r"\/").unwrap()),
            RawTokenType::Pow => Some(Regex::new(r"\^").unwrap()),
            RawTokenType::GreaterThan => Some(Regex::new(r">").unwrap()),
            RawTokenType::LessThan => Some(Regex::new(r"<").unwrap()),
            RawTokenType::GreaterThanEq => Some(Regex::new(r">=").unwrap()),
            RawTokenType::LessThanEq => Some(Regex::new(r"<=").unwrap()),
            RawTokenType::Equals => Some(Regex::new(r"=").unwrap()),
            RawTokenType::NotEquals => Some(Regex::new(r"!=").unwrap()),
            RawTokenType::Pipe => Some(Regex::new(r"\|").unwrap()),
            RawTokenType::Bang => Some(Regex::new(r"!").unwrap()),
            RawTokenType::Comma => Some(Regex::new(r",").unwrap()),
            RawTokenType::Semicolon => Some(Regex::new(r";").unwrap()),
            RawTokenType::Tick => Some(Regex::new(r"'").unwrap()),
            RawTokenType::LBrace => Some(Regex::new(r"{").unwrap()),
            RawTokenType::RBrace => Some(Regex::new(r"}").unwrap()),
            RawTokenType::LBracket => Some(Regex::new(r"\[").unwrap()),
            RawTokenType::RBracket => Some(Regex::new(r"\]").unwrap()),
            RawTokenType::LParen => Some(Regex::new(r"\(").unwrap()),
            RawTokenType::RParen => Some(Regex::new(r"\)").unwrap()),
            RawTokenType::Whitespace => Some(Regex::new(r"([\r\n\t\f\v ]+)").unwrap()),
            RawTokenType::Eol | RawTokenType::Eof => None,
        }
    }

    pub fn is_ignored(&self) -> bool {
        matches!(self, RawTokenType::Whitespace | RawTokenType::Eol | RawTokenType::Eof)
    }
}
