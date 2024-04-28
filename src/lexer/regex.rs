use proc_macros::IterEnum;
use fancy_regex::Regex;

#[derive(IterEnum)]
pub enum TokenRegex {
    Decimal,
    Whitespace,
    Eol,
}

impl TokenRegex {
    pub fn regex(self) -> Option<Regex> {
        match self {
            TokenRegex::Decimal => Some(Regex::new(r"([0-9]+\.?([0-9]+)?|\.[0-9]+)").unwrap()),
            TokenRegex::Whitespace => Some(Regex::new(r"([\r\n\t\f\v ]+)").unwrap()),
            TokenRegex::Eol => None,
        }
    }
}
