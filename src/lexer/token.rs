#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub ty: TokenType,
    // pub span: Span,
}


#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    Decimal(f64),
    Whitespace,
    Eol,
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
            (&Self::Eol, &Self::Eol)
        )
    }
}
