pub mod node;

use crate::{lexer::{raw_token::RawTokenType, token::{Token, TokenType}}, tteq, ttne};
use crate::prelude::*;
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
            current_token: tokens[0].clone(),
            tokens, 
            token_index: 0,
            advance_count: 0,
        }
    }

    fn advance(&mut self) {
        self.token_index += 1;
        self.advance_count += 1;
        if self.token_index < self.tokens.len() {
            self.current_token = self.tokens[self.token_index].clone();
        }
    }

    fn back(&mut self) {
        self.token_index -= 1;
        self.advance_count -= 1;
        if self.token_index < self.tokens.len() {
            self.current_token = self.tokens[self.token_index].clone();
        }
    }

    pub fn parse(&mut self) -> Result<Node> {
        let res = self.stmt()?;
        if self.current_token.ty != TokenType::Eof {
            return err!(Syntax, "expected '+', '-', '*', '/', or '^'", self.current_token.span);
        }
        Ok(res)
    }

    fn atom(&mut self) -> Result<Node> {
        let token = self.current_token.clone();

        if tteq!(token.ty => Decimal) {
            self.advance();
            return Ok(Node::Constant { token: token.clone() });
        }

        if tteq!(token.ty => Identifier) {
            self.advance();
            return Ok(Node::Variable { name: token.clone() });
        }

        if tteq!(token.ty => Add, Sub) {
            self.advance();
            let node = self.atom()?;
            return Ok(Node::UnaryOp { token: token.clone(), node: Box::new(node) });
        }

        if tteq!(token.ty => LParen) {
            self.advance();
            let expr = self.expr()?;
            if tteq!(self.current_token.ty => RParen) {
                self.advance();
                return Ok(expr);
            }
            return err!(Syntax, "expected ')'", self.current_token.span);
        }

        err!(Syntax, "expected decimal, '+', '-' or '('", self.current_token.span)
    }

    fn call(&mut self) -> Result<Node> {
        let call_start = self.current_token.span.pos_1;
        let atom = self.atom()?;

        if let Node::Variable { name } = atom.clone() {
            let mut params = Vec::new();
            if tteq!(self.current_token.ty => Colon) {
                self.advance();
                
                params.push(self.expr()?);

                while tteq!(self.current_token.ty => Colon) {
                    self.advance();
                    params.push(self.expr()?);
                }

                if ttne!(self.current_token.ty => LBracket) {
                    return err!(Syntax, "expected ':' or '['", self.current_token.span);
                }
            }

            if tteq!(self.current_token.ty => LBracket) {
                self.advance();
                let mut args = Vec::new();

                if tteq!(self.current_token.ty => RBracket) {
                    self.advance();
                } else {
                    args.push(self.expr()?);

                    while tteq!(self.current_token.ty => Comma) {
                        self.advance();
                        args.push(self.expr()?);
                    }

                    if ttne!(self.current_token.ty => RBracket) {
                        return err!(Syntax, "expected ',' or ']'", self.current_token.span);
                    }
                    self.advance();
                }

                return Ok(Node::Call { name, args, params, span: Span::new(call_start, self.current_token.span.pos_2 ) });
            }
        }

        Ok(atom)
    }

    fn factor(&mut self) -> Result<Node> {
        self.bin_op(Self::call, Self::call, &[TokenType::Pow])
    }

    fn term(&mut self) -> Result<Node> {
        let mut left = self.factor()?;

        loop {
            let mut should_break = false;

            if tteq!(self.current_token.ty => Mul, Div) {
                let token = self.current_token.clone();
                self.advance();
                let right = self.factor()?;
                left = Node::BinaryOp { token, left: Box::new(left), right: Box::new(right) }
            } else {
                should_break = true;
            }

            if tteq!(self.current_token.ty => Add, Sub) {
                break;
            }

            let cur_span = self.current_token.span;
            match self.factor() {
                Ok(right) => {
                    left = Node::BinaryOp {
                        token: Token { ty: TokenType::Mul, span: cur_span.move_by(-1) },
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                },
                Err(err) => {
                    if cur_span == self.current_token.span {
                        if should_break { break };
                    } else {
                        return Err(err);
                    }
                }
            }
        }

        Ok(left)
    }

    fn expr(&mut self) -> Result<Node> {
        self.bin_op(Self::term, Self::term, &[TokenType::Add, TokenType::Sub])
    }

    fn stmt(&mut self) -> Result<Node> {
        self.bin_op(Self::expr, Self::expr, &[
            TokenType::Equals,
            TokenType::NotEquals,
            TokenType::GreaterThan,
            TokenType::LessThan,
            TokenType::GreaterThanEq,
            TokenType::LessThanEq,
        ])
    }

    fn bin_op<F, G>(&mut self, left_fn: F, right_fn: G, tokens: &[TokenType]) -> Result<Node> 
    where 
        F: Fn(&mut Self) -> Result<Node>,
        G: Fn(&mut Self) -> Result<Node>
    {
        let mut left = left_fn(self)?;

        while tokens.iter().any(|t| t == &self.current_token.ty) {
            let token = self.current_token.clone();
            self.advance();
            let right = right_fn(self)?;
            left = Node::BinaryOp { token, left: Box::new(left), right: Box::new(right) };
        }

        Ok(left)
    }
}
