use std::fmt::{Display, Formatter};

use is_macro::Is;

use crate::lex::{ShallowTokenKind, Token};

#[derive(Debug)]
pub struct Error {
    // TODO: Make this a span
    pub index: usize,
    pub kind: ErrorKind,
}

#[derive(Debug, Is)]
pub enum ErrorKind {
    ExpectedToken {
        expected: ShallowTokenKind,
        received: Option<Token>,
    },
    ExpectedBinaryOperator {
        received: Option<Token>,
    },
    ExpectedLiteral {
        received: Option<Token>,
    },
}

impl Error {
    /// Adjusts [Self::index] by an index.
    pub fn relative_to(mut self, by: usize) -> Self {
        self.index += by;
        self
    }

    pub fn expected_token(
        at_index: usize,
        expected: ShallowTokenKind,
        received: Option<Token>,
    ) -> Self {
        Self {
            index: at_index,
            kind: ErrorKind::ExpectedToken { expected, received },
        }
    }

    pub fn expected_binary_operator(at_index: usize, received: Option<Token>) -> Self {
        Self {
            index: at_index,
            kind: ErrorKind::ExpectedBinaryOperator { received },
        }
    }

    pub fn expected_literal(at_index: usize, received: Option<Token>) -> Self {
        Self {
            index: at_index,
            kind: ErrorKind::ExpectedLiteral { received },
        }
    }
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
