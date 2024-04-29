#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub index: usize,
    pub line: usize,
    pub column: usize,
}

#[allow(dead_code)]
impl Position {
    pub fn new(index: usize, line: usize, column: usize) -> Self {
        Self {
            index, 
            line, 
            column,
        }
    }

    /// Advances the position by one given some text.
    pub fn advance(mut self, text: &str) -> Self {
        self.index += text.len();
        if text.contains('\n') {
            self.line += 1;
            self.column = text.split('\n').last().unwrap().chars().count() + 1;
        } else {
            self.column += text.chars().count();
        }
        self
    }

    /// Advances the position by one given some text.
    pub fn advance_mut(&mut self, text: &str) {
        self.index += text.len();
        if text.contains('\n') {
            self.line += 1;
            self.column = text.split('\n').last().unwrap().chars().count() + 1;
        } else {
            self.column += text.chars().count();
        }
    }

    /// Advances the position by one column, ignoring anything in the text (like linebreaks).
    pub fn forward(mut self) -> Self {
        self.column += 1;
        self
    }

    /// Advances the position by `amount` columns, ignoring anything in the text (like linebreaks).
    pub fn forward_by(mut self, amount: usize) -> Self {
        self.column += amount;
        self
    }

    /// Backtracks the position given some text.
    pub fn backtrack(mut self, text: &str) -> Self {
        self.index -= text.len();
        if text.contains('\n') {
            self.line += 1;
            self.column = text.split('\n').last().unwrap().chars().count() + 1;
        } else {
            self.column += text.chars().count();
        }
        self
    }

    /// Backtracks the position given some text.
    pub fn backtrack_mut(&mut self, text: &str) {
        self.index += text.len();
        if text.contains('\n') {
            self.line += 1;
            self.column = text.split('\n').last().unwrap().chars().count() + 1;
        } else {
            self.column += text.chars().count();
        }
    }

    /// Backtracks the position by one column, ignoring anything in the text (like linebreaks).
    pub fn backward(mut self) -> Self {
        self.column -= 1;
        self
    }

    /// Advances the position by `amount` columns, ignoring anything in the text (like linebreaks).
    pub fn backward_by(mut self, amount: usize) -> Self {
        self.column -= amount;
        self
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
            index: 0,
            line: 1,
            column: 1,
        }
    }
}
