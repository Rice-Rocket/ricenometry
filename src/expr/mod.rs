use crate::{lexer::token::TokenType, parser::node::Node};

pub mod simplify;

pub enum Expr {
    Constant(Constant),
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
    Quotient {
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


#[derive(Debug, Clone, Copy)]
pub enum Constant {
    Integer(u64),
    Decimal(f64),
    Ratio(u64, u64),
    Radical(u64, u64),
}

impl Constant {
    pub fn reduce(self) -> Self {
        match self {
            Self::Integer(n) => Self::Integer(n),
            Self::Decimal(v) => Self::Decimal(v),
            Self::Ratio(n, d) => Self::Ratio(n, d),
            Self::Radical(i, r) => Self::Ratio(i, r),
        }
    }
}


impl From<Node> for Expr {
    fn from(value: Node) -> Self {
        match value {
            Node::Constant { token } => Expr::Constant(if let TokenType::Decimal(v) = token.ty {
                if v.fract() < 2e-6 {
                    Constant::Integer(v.round() as u64)
                } else {
                    Constant::Decimal(v)
                }
            } else { unreachable!() }),
            Node::Variable { name } => Expr::Variable(format!("{}", name.ty)),
            Node::BinaryOp { token, left, right } => match token.ty {
                TokenType::Add => Expr::Sum { left: Box::new(Expr::from(*left)), right: Box::new(Expr::from(*right)) },
                TokenType::Sub => Expr::Difference { left: Box::new(Expr::from(*left)), right: Box::new(Expr::from(*right)) },
                TokenType::Mul => Expr::Product { left: Box::new(Expr::from(*left)), right: Box::new(Expr::from(*right)) },
                TokenType::Div => Expr::Quotient { numerator: Box::new(Expr::from(*left)), denominator: Box::new(Expr::from(*right)) },
                TokenType::Pow => Expr::Power { base: Box::new(Expr::from(*left)), exp: Box::new(Expr::from(*right)) },
                TokenType::GreaterThan => Expr::GreaterThan { left: Box::new(Expr::from(*left)), right: Box::new(Expr::from(*right)) },
                TokenType::LessThan => Expr::LessThan { left: Box::new(Expr::from(*left)), right: Box::new(Expr::from(*right)) },
                TokenType::GreaterThanEq => Expr::GreaterThanEq { left: Box::new(Expr::from(*left)), right: Box::new(Expr::from(*right)) },
                TokenType::LessThanEq => Expr::LessThanEq { left: Box::new(Expr::from(*left)), right: Box::new(Expr::from(*right)) },
                TokenType::Equals => Expr::Equals { left: Box::new(Expr::from(*left)), right: Box::new(Expr::from(*right)) },
                TokenType::NotEquals => Expr::NotEquals { left: Box::new(Expr::from(*left)), right: Box::new(Expr::from(*right)) },
                _ => unreachable!(),
            },
            Node::UnaryOp { token, node } => Expr::Negation(Box::new(Expr::from(*node))),
            Node::Call { name, args, span } => todo!(),
        }
    }
}
