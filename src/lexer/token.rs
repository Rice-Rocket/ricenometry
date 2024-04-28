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
        write!(f, "[{}]", self.ty)
    }
}


#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    Decimal(f64),
    Whitespace,
    Eol,
    Eof,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &TokenType::Decimal(val) => write!(f, "Decimal({})", val),

            no_val => write!(f, "{:?}", no_val)
        }
    }
}

impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        matches!((self, other), 
            (&Self::Decimal(_), &Self::Decimal(_)) | 
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
            RawTokenType::Whitespace => Self::Whitespace,
            RawTokenType::Eol => Self::Eol,
            RawTokenType::Eof => Self::Eof,
        }
    }
}
