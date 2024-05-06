pub mod token;
pub mod raw_token;
pub mod regex_set;

use fancy_regex::Regex;
use token::{Token, TokenType};
use raw_token::RawTokenType;
use regex_set::RegexSet;

use crate::prelude::*;


pub struct Lexer<'a> {
    pub toks: Vec<Option<RawTokenType>>,
    pub regexes: Vec<Regex>,
    pub regex_set: RegexSet,
    pub source: &'a str,
    pub position: Position,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let regexes: Vec<Regex> = RawTokenType::iter_fields()
            .flat_map(|t| t.regex().into_iter())
            .collect();
        let regex_set = RegexSet::new(regexes.clone());

        Self {
            toks: RawTokenType::iter_fields()
                .filter(|t| t.regex().is_some())
                .map(|tok| match tok.is_ignored() {
                    true => None,
                    false => Some(tok),
                })
                .collect(),
            regexes,
            regex_set,
            source,
            position: Position::default(),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        loop {
            if self.position.index == self.source.len() {
                break;
            }
            let string = &self.source[self.position.index..];
            
            let match_set = self.regex_set.matches(string);
            let Some((start, len, i)) = match_set
                .iter()
                .map(|i| {
                    let m = self.regexes[i].find(string).unwrap().unwrap();
                    (m.start(), m.end() - m.start(), i)
                })
                .max_by(|(start1, len1, _), (start2, len2, _)| match start2.cmp(start1) {
                    std::cmp::Ordering::Equal => len1.cmp(len2),
                    not_eq => not_eq,
                })
            else {
                return err!(
                    UnknownCharacter, 
                    "'{}' is not a valid character", 
                    Span::new_single(self.position);
                    &self.source[self.position.index..self.position.index + 1]
                );
            };
            
            if start != 0 {
                return err!(
                    UnknownCharacter,
                    "'{}' is not a valid character",
                    Span::new_single(self.position);
                    &self.source[self.position.index..self.position.index + 1]
                )
            }
            
            let span = start + self.position.index..start + self.position.index + len;
            let text = &self.source[span.clone()];

            if let Some(tok) = self.toks[i] {
                tokens.push(
                    Token::new(
                        tok,
                        Span::new(
                            self.position.advance(&self.source[self.position.index..self.position.index + start]),
                            self.position.advance(text),
                        ),
                        text
                    )
                )
            }
            self.position.advance_mut(text);
        }

        tokens.push(Token {
            ty: TokenType::Eof, 
            span: Span::new_single(self.position), 
        });
        Ok(tokens)
    }
}
