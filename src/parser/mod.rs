pub mod node;

use crate::{error::Error, lexer::{token::{Token, TokenType}, raw_token::RawTokenType}, tteq};
use node::Node;

pub struct Parser {
    pub tokens: Vec<Token>,
    pub token_index: usize,
    pub current_token: Token,
    advance_count: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            current_token: tokens[0],
            tokens, 
            token_index: 0,
            advance_count: 0,
        }
    }

    fn advance(&mut self) {
        self.token_index += 1;
        self.advance_count += 1;
        if self.token_index < self.tokens.len() {
            self.current_token = self.tokens[self.token_index];
        }
    }

    fn back(&mut self) {
        self.token_index -= 1;
        self.advance_count -= 1;
        if self.token_index < self.tokens.len() {
            self.current_token = self.tokens[self.token_index]
        }
    }

    pub fn parse(&mut self) -> Result<Node, Error> {
        let res = self.expr()?;
        if self.current_token.ty != TokenType::Eof {
            return Err(Error::Syntax("expected '+', '-', '*', '/', or '^'".to_string(), self.current_token.span))
        }
        Ok(res)
    }

    fn factor(&mut self) -> Result<Node, Error> {
        let token = self.current_token;

        if tteq!(token.ty => Decimal) {
            self.advance();
            return Ok(Node::Decimal { token });
        }

        if tteq!(token.ty => Add, Sub) {
            self.advance();
            let node = self.factor()?;
            return Ok(Node::UnaryOp { token, node: Box::new(node) });
        }

        if tteq!(token.ty => LParen) {
            self.advance();
            let expr = self.expr()?;
            if tteq!(self.current_token.ty => RParen) {
                self.advance();
                return Ok(expr);
            }
            return Err(Error::Syntax("expected ')'".to_string(), self.current_token.span));
        }

        Err(Error::Syntax("expected decimal, '+', '-' or '('".to_string(), self.current_token.span))
    }

    fn term(&mut self) -> Result<Node, Error> {
        self.bin_op(Self::factor, Self::factor, &[TokenType::Mul, TokenType::Div])
    }

    fn expr(&mut self) -> Result<Node, Error> {
        self.bin_op(Self::term, Self::term, &[TokenType::Add, TokenType::Sub])
    }

    fn bin_op<F, G>(&mut self, left_fn: F, right_fn: G, tokens: &[TokenType]) -> Result<Node, Error> 
    where 
        F: Fn(&mut Self) -> Result<Node, Error>,
        G: Fn(&mut Self) -> Result<Node, Error>
    {
        let mut left = left_fn(self)?;

        while tokens.iter().any(|t| t == &self.current_token.ty) {
            let token = self.current_token;
            self.advance();
            let right = right_fn(self)?;
            left = Node::BinaryOp { token, left: Box::new(left), right: Box::new(right) };
        }

        Ok(left)
    }
}
