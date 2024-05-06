use proc_macros::FieldConstructor;

use crate::{lexer::token::TokenType, parser::node::Node};
use crate::prelude::*;

pub mod simplify;

#[derive(Clone, FieldConstructor)]
pub enum Expr {
    Integer(i64),
    Decimal(f64),
    Variable(String),

    Negation(Box<Expr>),

    Sum {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Difference {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Product {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Ratio {
        numerator: Box<Expr>,
        denominator: Box<Expr>,
    },
    Power {
        base: Box<Expr>,
        exp: Box<Expr>,
    },
    Root {
        index: Box<Expr>,
        radicand: Box<Expr>,
    },

    Equals {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    NotEquals {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    GreaterThan {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    LessThan {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    GreaterThanEq {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    LessThanEq {
        left: Box<Expr>,
        right: Box<Expr>,
    },
}


impl Expr {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
    
    pub fn convert(value: Node) -> Result<Self> {
        Ok(match value {
            Node::Constant { token } => if let TokenType::Decimal(v) = token.ty {
                if v.fract() < 2e-6 {
                    Expr::Integer(v.round() as i64)
                } else {
                    Expr::Decimal(v)
                }
            } else { unreachable!() },
            Node::Variable { name } => Expr::Variable(format!("{}", name.ty)),
            Node::BinaryOp { token, left, right } => match token.ty {
                TokenType::Add => Expr::Sum { left: Box::new(Expr::convert(*left)?), right: Box::new(Expr::convert(*right)?) },
                TokenType::Sub => Expr::Difference { left: Box::new(Expr::convert(*left)?), right: Box::new(Expr::convert(*right)?) },
                TokenType::Mul => Expr::Product { left: Box::new(Expr::convert(*left)?), right: Box::new(Expr::convert(*right)?) },
                TokenType::Div => Expr::Ratio { numerator: Box::new(Expr::convert(*left)?), denominator: Box::new(Expr::convert(*right)?) },
                TokenType::Pow => Expr::Power { base: Box::new(Expr::convert(*left)?), exp: Box::new(Expr::convert(*right)?) },
                TokenType::GreaterThan => Expr::GreaterThan { left: Box::new(Expr::convert(*left)?), right: Box::new(Expr::convert(*right)?) },
                TokenType::LessThan => Expr::LessThan { left: Box::new(Expr::convert(*left)?), right: Box::new(Expr::convert(*right)?) },
                TokenType::GreaterThanEq => Expr::GreaterThanEq { left: Box::new(Expr::convert(*left)?), right: Box::new(Expr::convert(*right)?) },
                TokenType::LessThanEq => Expr::LessThanEq { left: Box::new(Expr::convert(*left)?), right: Box::new(Expr::convert(*right)?) },
                TokenType::Equals => Expr::Equals { left: Box::new(Expr::convert(*left)?), right: Box::new(Expr::convert(*right)?) },
                TokenType::NotEquals => Expr::NotEquals { left: Box::new(Expr::convert(*left)?), right: Box::new(Expr::convert(*right)?) },
                _ => unreachable!(),
            },
            Node::UnaryOp { token, node } => Expr::Negation(Box::new(Expr::convert(*node)?)),
            Node::Call { name, params, args, span } => if let TokenType::Identifier(name) = name.ty {
                match name.as_str() {
                    "sqrt" => {
                        if args.len() != 1 { return err!(InvalidCall, "expected 1 argument, got {}", span; args.len()) };
                        if !params.is_empty() { return err!(InvalidCall, "expected 0 parameters, got {}", span; params.len()) };
                        Expr::Root { index: Box::new(Expr::Integer(2)), radicand: Box::new(Expr::convert(args[0].clone())?) }
                    },
                    "cbrt" => {
                        if args.len() != 1 { return err!(InvalidCall, "expected 1 argument, got {}", span; args.len()) };
                        if !params.is_empty() { return err!(InvalidCall, "expected 0 parameters, got {}", span; params.len()) };
                        Expr::Root { index: Box::new(Expr::Integer(3)), radicand: Box::new(Expr::convert(args[0].clone())?) }
                    },
                    "root" => {
                        if args.len() != 1 { return err!(InvalidCall, "expected 1 argument, got {}", span; args.len()) };
                        if params.len() != 1 { return err!(InvalidCall, "expected 1 parameters, got {}", span; params.len()) };
                        Expr::Root { index: Box::new(Expr::convert(params[0].clone())?), radicand: Box::new(Expr::convert(args[0].clone())?) }
                    },
                    _user_fn => todo!(),
                }
            } else { unreachable!() },
        })
    }
}


impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Integer(c) => write!(f, "{}", c),
            Expr::Decimal(v) => write!(f, "{}", v),
            Expr::Variable(s) => write!(f, "{}", s),
            Expr::Negation(node) => write!(f, "-{}", node),
            Expr::Sum { left, right } => write!(f, "({} + {})", left, right),
            Expr::Difference { left, right } => write!(f, "({} - {})", left, right),
            Expr::Product { left, right } => write!(f, "{}{}", left, right),
            Expr::Ratio { numerator, denominator } => write!(f, "({} / {})", numerator, denominator),
            Expr::Power { base, exp } => write!(f, "({} ^ {})", base, exp),
            Expr::Root { index, radicand } => write!(f, "{}√{}", utils::superscript(&format!("{}", index)), radicand),
            Expr::Equals { left, right } => write!(f, "{} = {}", left, right),
            Expr::NotEquals { left, right } => write!(f, "{} != {}", left, right),
            Expr::GreaterThan { left, right } => write!(f, "{} > {}", left, right),
            Expr::LessThan { left, right } => write!(f, "{} < {}", left, right),
            Expr::GreaterThanEq { left, right } => write!(f, "{} >= {}", left, right),
            Expr::LessThanEq { left, right } => write!(f, "{} <= {}", left, right),
        }
    }
}
