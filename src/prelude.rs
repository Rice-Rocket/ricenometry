pub use crate::error::{Error, span::Span, position::Position};
pub use crate::err;
pub(crate) use crate::utils;

pub type Result<T> = core::result::Result<T, Error>;

pub struct Wrap<T>(pub T);
