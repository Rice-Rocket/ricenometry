pub use crate::error::{Error, span::Span, position::Position};

pub type Result<T> = core::result::Result<T, Error>;

pub struct Wrap<T>(pub T);
