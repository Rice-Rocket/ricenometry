use std::collections::VecDeque;
use termion::color;

use crate::lexer::token::Token;

#[derive(Clone)]
pub enum Node {
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

impl Node {
    fn fmt_root(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Decimal { token } => write!(f, "{}{:?}{}", color::Fg(color::Yellow), token.ty, color::Fg(color::Reset)),
            Node::BinaryOp { token, .. } => write!(f, "{}{:?}{}", color::Fg(color::LightGreen), token.ty, color::Fg(color::Reset)),
            Node::UnaryOp { token, .. } => write!(f, "{}{:?}{}", color::Fg(color::LightBlue), token.ty, color::Fg(color::Reset)),
        }
    }

    fn leaves(&self) -> Vec<Node> {
        match self {
            Node::Decimal { .. } => vec![],
            Node::BinaryOp { left, right, .. } => vec![*left.clone(), *right.clone()],
            Node::UnaryOp { node, .. } => vec![*node.clone()],
        }
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Decimal { token } => write!(f, "{}", token.ty),
            Node::BinaryOp { token, left, right } => write!(f, "({}) {} ({})", left, token.ty, right),
            Node::UnaryOp { token, node } => write!(f, "{}{}", token.ty, node),
        }
    }
}


type DisplayQueue = VecDeque<(bool, Node, Vec<bool>)>;

const GLYPH_MIDDLE_ITEM: &str = "├";
const GLYPH_LAST_ITEM: &str = "└";
const GLYPH_ITEM_INDENT: &str = "── ";
const GLYPH_MIDDLE_SKIP: &str = "│";
const GLYPH_LAST_SKIP: &str = " ";
const GLYPH_SKIP_INDENT: &str = "  ";

fn enqueue_leaves(
    queue: &mut DisplayQueue,
    parent: &Node,
    spaces: Vec<bool>
) {
    for (i, leaf) in parent.leaves().iter().rev().enumerate() {
        let last = i == 0;
        queue.push_front((last, leaf.clone(), spaces.clone()));
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_root(f)?;
        writeln!(f)?;
        
        let mut queue = DisplayQueue::new();
        enqueue_leaves(&mut queue, self, Vec::new());

        while let Some((last, leaf, spaces)) = queue.pop_front() {
            let mut prefix = (
                if last {
                    GLYPH_LAST_ITEM
                } else {
                    GLYPH_MIDDLE_ITEM
                },
                GLYPH_ITEM_INDENT
            );

            let rest_prefix = (
                if last {
                    GLYPH_LAST_SKIP
                } else {
                    GLYPH_MIDDLE_SKIP
                },
                GLYPH_SKIP_INDENT
            );

            let root = format!("{}", leaf);
            for line in root.lines() {
                write!(f, "{}", color::Fg(color::LightBlack))?;
                for s in spaces.as_slice() {
                    if *s {
                        GLYPH_LAST_SKIP.fmt(f)?;
                    } else {
                        GLYPH_MIDDLE_SKIP.fmt(f)?;
                    }
                    GLYPH_SKIP_INDENT.fmt(f)?;
                }
                prefix.0.fmt(f)?;
                prefix.1.fmt(f)?;
                write!(f, "{}", color::Fg(color::Reset))?;
                line.fmt(f)?;
                writeln!(f)?;
                prefix = rest_prefix;
            }
        }

        Ok(())
    }
}
