use proc_macros::IterEnum;
use fancy_regex::Regex;

#[derive(Debug, Clone, Copy, IterEnum)]
pub enum RawTokenType {
    Decimal,
    Whitespace,
    Eol,
    Eof,
}

impl RawTokenType {
    pub fn regex(self) -> Option<Regex> {
        match self {
            RawTokenType::Decimal => Some(Regex::new(r"([0-9]+\.?([0-9]+)?|\.[0-9]+)").unwrap()),
            RawTokenType::Whitespace => Some(Regex::new(r"([\r\n\t\f\v ]+)").unwrap()),
            RawTokenType::Eol | RawTokenType::Eof => None,
        }
    }

    pub fn is_ignored(&self) -> bool {
        matches!(self, RawTokenType::Whitespace | RawTokenType::Eol | RawTokenType::Eof)
    }
}
