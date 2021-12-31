use std::fmt::Display;

pub enum Error {
    WrongParenthesis { instruction: usize },
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::WrongParenthesis { instruction } => {
                write!(f, "error wrong parenthesis on line {}", instruction)
            }
        }
    }
}

pub type Result<A> = std::result::Result<A, Error>;
