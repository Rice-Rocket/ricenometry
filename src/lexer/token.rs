use proc_macros::StringifyEnum;

use crate::error::span::Span;

use super::raw_token::RawTokenType;

#[derive(Clone, Copy)]
pub struct Token {
    pub ty: TokenType,
    pub span: Span,
}

impl Token {
    pub fn new(raw: RawTokenType, span: Span, text: &str) -> Self {
        let ty = match raw {
            RawTokenType::Decimal => TokenType::Decimal(text.parse().unwrap()),
            _ => raw.into()
        };

        Self {
            ty,
            span,
        }
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}]", self.ty)
    }
}


#[derive(Clone, Copy, StringifyEnum)]
pub enum TokenType {
    Decimal(f64),

    /// +
    Add,
    /// -
    Sub,
    /// *
    Mul,
    /// /
    Div,
    /// ^
    Pow,

    /// >
    GreaterThan,
    /// <
    LessThan,
    /// >=
    GreaterThanEq,
    /// <=
    LessThanEq,
    /// =
    Equals,
    /// !=
    NotEquals,

    /// |
    Pipe,
    /// !
    Bang,
    /// ,
    Comma,
    /// ;
    Semicolon,
    /// '
    Tick,

    /// {
    LBrace,
    /// }
    RBrace,
    /// [
    LBracket,
    /// ]
    RBracket,
    /// (
    LParen,
    /// )
    RParen,

    Whitespace,
    Eol,
    Eof,
}

impl std::fmt::Debug for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &TokenType::Decimal(val) => write!(f, "Decimal({})", val),

            no_val => write!(f, "{}", no_val.stringify_field())
        }
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &TokenType::Decimal(val) => write!(f, "{}", val),

            no_val => write!(f, "{}", no_val.stringify_pretty())
        }
    }
}

impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        matches!((self, other), 
            (&Self::Decimal(_), &Self::Decimal(_)) | 
            (&Self::Add, &Self::Add) |
            (&Self::Sub, &Self::Sub) |
            (&Self::Mul, &Self::Mul) |
            (&Self::Div, &Self::Div) |
            (&Self::Pow, &Self::Pow) |
            (&Self::GreaterThan, &Self::GreaterThan) |
            (&Self::LessThan, &Self::LessThan) |
            (&Self::GreaterThanEq, &Self::GreaterThanEq) |
            (&Self::LessThanEq, &Self::LessThanEq) |
            (&Self::Equals, &Self::Equals) |
            (&Self::NotEquals, &Self::NotEquals) |
            (&Self::Pipe, &Self::Pipe) |
            (&Self::Bang, &Self::Bang) |
            (&Self::Comma, &Self::Comma) |
            (&Self::Semicolon, &Self::Semicolon) |
            (&Self::Tick, &Self::Tick) |
            (&Self::LBrace, &Self::LBrace) |
            (&Self::RBrace, &Self::RBrace) |
            (&Self::LBracket, &Self::LBracket) |
            (&Self::RBracket, &Self::RBracket) |
            (&Self::LParen, &Self::LParen) |
            (&Self::RParen, &Self::RParen) |
            (&Self::Whitespace, &Self::Whitespace) | 
            (&Self::Eol, &Self::Eol) |
            (&Self::Eof, &Self::Eof)
        )
    }
}

impl From<RawTokenType> for TokenType {
    fn from(value: RawTokenType) -> Self {
        match value {
            RawTokenType::Decimal => Self::Decimal(0.0),
            RawTokenType::Add => Self::Add,
            RawTokenType::Sub => Self::Sub,
            RawTokenType::Mul => Self::Mul,
            RawTokenType::Div => Self::Div,
            RawTokenType::Pow => Self::Pow,
            RawTokenType::GreaterThan => Self::GreaterThan,
            RawTokenType::LessThan => Self::LessThan,
            RawTokenType::GreaterThanEq => Self::GreaterThanEq,
            RawTokenType::LessThanEq => Self::LessThanEq,
            RawTokenType::Equals => Self::Equals,
            RawTokenType::NotEquals => Self::NotEquals,
            RawTokenType::Pipe => Self::Pipe,
            RawTokenType::Bang => Self::Bang,
            RawTokenType::Comma => Self::Comma,
            RawTokenType::Semicolon => Self::Semicolon,
            RawTokenType::Tick => Self::Tick,
            RawTokenType::LBrace => Self::LBrace,
            RawTokenType::RBrace => Self::RBrace,
            RawTokenType::LBracket => Self::LBracket,
            RawTokenType::RBracket => Self::RBracket,
            RawTokenType::LParen => Self::LParen,
            RawTokenType::RParen => Self::RParen,
            RawTokenType::Whitespace => Self::Whitespace,
            RawTokenType::Eol => Self::Eol,
            RawTokenType::Eof => Self::Eof,
        }
    }
}

impl PartialEq<RawTokenType> for TokenType {
    fn eq(&self, rhs: &RawTokenType) -> bool {
        match (self, rhs) {
            (&Self::Decimal(_), &RawTokenType::Decimal) => true,
            (&Self::Add, &RawTokenType::Add) => true,
            (&Self::Sub, &RawTokenType::Sub) => true,
            (&Self::Mul, &RawTokenType::Mul) => true,
            (&Self::Div, &RawTokenType::Div) => true,
            (&Self::Pow, &RawTokenType::Pow) => true,
            (&Self::GreaterThan, &RawTokenType::GreaterThan) => true,
            (&Self::LessThan, &RawTokenType::LessThan) => true,
            (&Self::GreaterThanEq, &RawTokenType::GreaterThanEq) => true,
            (&Self::LessThanEq, &RawTokenType::LessThanEq) => true,
            (&Self::Equals, &RawTokenType::Equals) => true,
            (&Self::NotEquals, &RawTokenType::NotEquals) => true,
            (&Self::Pipe, &RawTokenType::Pipe) => true,
            (&Self::Bang, &RawTokenType::Bang) => true,
            (&Self::Comma, &RawTokenType::Comma) => true,
            (&Self::Semicolon, &RawTokenType::Semicolon) => true,
            (&Self::Tick, &RawTokenType::Tick) => true,
            (&Self::LBrace, &RawTokenType::LBrace) => true,
            (&Self::RBrace, &RawTokenType::RBrace) => true,
            (&Self::LBracket, &RawTokenType::LBracket) => true,
            (&Self::RBracket, &RawTokenType::RBracket) => true,
            (&Self::LParen, &RawTokenType::LParen) => true,
            (&Self::RParen, &RawTokenType::RParen) => true,
            (&Self::Whitespace, &RawTokenType::Whitespace) => true,
            (&Self::Eol, &RawTokenType::Eol) => true,
            (&Self::Eof, &RawTokenType::Eof) => true,
            _ => false,
        }
    }
}


#[macro_export]
macro_rules! tteq {
    ($tty:expr => $match0:ident $(,$match:ident)* $(,)*) => {
        ($tty == RawTokenType::$match0) $(
            || ($tty == RawTokenType::$match)
        )*
    }
}
