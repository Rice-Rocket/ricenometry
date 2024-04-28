use crate::lexer::token::Token;

#[derive(Clone)]
pub enum Node {
    Unit {
        token: Token,
    },
    Decimal {
        token: Token,
    },
    BinaryOp {
        token: Token,
        left: Box<Node>,
        right: Box<Node>,
    },
    UnaryOp {
        token: Token,
        node: Box<Node>,
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Unit { token: _ } => write!(f, "()"),
            Node::Decimal { token } => write!(f, "{}", token.ty),
            Node::BinaryOp { token, left, right } => write!(f, "({}) {} ({})", left, token.ty, right),
            Node::UnaryOp { token, node } => write!(f, "{}{}", token.ty, node),
        }
    }
}
