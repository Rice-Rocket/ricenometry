use proc_macros::StringifyEnum;

use crate::error::span::Span;

use super::raw_token::RawTokenType;

#[derive(Clone)]
pub struct Token {
    pub ty: TokenType,
    pub span: Span,
}

impl Token {
    pub fn new(raw: RawTokenType, span: Span, text: &str) -> Self {
        let ty = match raw {
            RawTokenType::Decimal => TokenType::Decimal(text.parse().unwrap()),
            RawTokenType::Identifier => TokenType::Identifier(text.to_string()),
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


#[derive(Clone, StringifyEnum)]
pub enum TokenType {
    Decimal(f64),
    Identifier(String),

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
    /// :
    Colon,
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
            TokenType::Decimal(val) => write!(f, "Decimal({})", val),
            TokenType::Identifier(val) => write!(f, "Ident({})", val),

            no_val => write!(f, "{}", no_val.stringify_field())
        }
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Decimal(val) => write!(f, "{}", val),
            TokenType::Identifier(val) => write!(f, "{}", val),

            no_val => write!(f, "{}", no_val.stringify_pretty())
        }
    }
}

impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        matches!((self, other), 
            (&Self::Decimal(_), &Self::Decimal(_)) | 
            (&Self::Identifier(_), &Self::Identifier(_)) |
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
            (&Self::Colon, &Self::Colon) |
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
            RawTokenType::Identifier => Self::Identifier(String::new()),
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
            RawTokenType::Colon => Self::Colon,
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
        matches!((self, rhs), 
            (&Self::Decimal(_), &RawTokenType::Decimal) | 
            (&Self::Identifier(_), &RawTokenType::Identifier) |
            (&Self::Add, &RawTokenType::Add) | 
            (&Self::Sub, &RawTokenType::Sub) | 
            (&Self::Mul, &RawTokenType::Mul) | 
            (&Self::Div, &RawTokenType::Div) | 
            (&Self::Pow, &RawTokenType::Pow) | 
            (&Self::GreaterThan, &RawTokenType::GreaterThan) | 
            (&Self::LessThan, &RawTokenType::LessThan) | 
            (&Self::GreaterThanEq, &RawTokenType::GreaterThanEq) | 
            (&Self::LessThanEq, &RawTokenType::LessThanEq) | 
            (&Self::Equals, &RawTokenType::Equals) | 
            (&Self::NotEquals, &RawTokenType::NotEquals) | 
            (&Self::Pipe, &RawTokenType::Pipe) | 
            (&Self::Bang, &RawTokenType::Bang) | 
            (&Self::Comma, &RawTokenType::Comma) | 
            (&Self::Semicolon, &RawTokenType::Semicolon) | 
            (&Self::Colon, &RawTokenType::Colon) | 
            (&Self::Tick, &RawTokenType::Tick) | 
            (&Self::LBrace, &RawTokenType::LBrace) | 
            (&Self::RBrace, &RawTokenType::RBrace) | 
            (&Self::LBracket, &RawTokenType::LBracket) | 
            (&Self::RBracket, &RawTokenType::RBracket) | 
            (&Self::LParen, &RawTokenType::LParen) | 
            (&Self::RParen, &RawTokenType::RParen) | 
            (&Self::Whitespace, &RawTokenType::Whitespace) | 
            (&Self::Eol, &RawTokenType::Eol) | 
            (&Self::Eof, &RawTokenType::Eof))
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

#[macro_export]
macro_rules! ttne {
    ($tty:expr => $match0:ident $(,$match:ident)* $(,)*) => {
        ($tty != RawTokenType::$match0) $(
            && ($tty != RawTokenType::$match)
        )*
    }
}
