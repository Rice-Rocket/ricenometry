use std::ops::{Add, Range};

use super::position::Position;

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub pos_1: Position,
    pub pos_2: Position,
}

#[allow(dead_code)]
impl Span {
    pub fn new(pos_1: Position, pos_2: Position) -> Self {
        Self {
            pos_1,
            pos_2,
        }
    }

    pub fn new_single(pos: Position) -> Self {
        Self {
            pos_1: pos,
            pos_2: pos,
        }
    }

    pub fn lines(&self) -> Range<usize> {
        self.pos_1.line..self.pos_2.line + 1
    }

    pub fn add_back(self, amount: usize) -> Self {
        Self {
            pos_2: self.pos_2.forward_by(amount),
            ..self
        }
    }

    pub fn add_front(self, amount: usize) -> Self {
        let mut pos_1 = self.pos_1;
        pos_1.column -= amount;

        Self {
            pos_1: self.pos_1.backward_by(amount),
            ..self
        }
    }

    pub fn extend(self, rhs: Self) -> Self {
        Self {
            pos_1: self.pos_1,
            pos_2: rhs.pos_2,
        }
    }
}

impl Add<Span> for Span {
    type Output = Span;
    fn add(self, rhs: Span) -> Self::Output {
        self.extend(rhs)
    }
}
