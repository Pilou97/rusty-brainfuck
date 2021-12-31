use std::fmt::Display;

/// Error encountered during parsing and execution time
pub enum Error {
    WrongParenthesis { instruction: usize },
    SegmentationFault(String),
    Overflow,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::WrongParenthesis { instruction } => {
                write!(f, "error wrong parenthesis on line {}", instruction)
            }
            Error::SegmentationFault(reason) => {
                write!(f, "segmentation fault: {}", reason)
            }
            Error::Overflow => write!(f, "overflow"),
        }
    }
}

pub type Result<A> = std::result::Result<A, Error>;
