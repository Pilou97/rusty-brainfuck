use crate::error::{Error, Result};
use std::fmt::Display;

/// The isntrucions of the brainfuck langugages
#[derive(Clone)]
pub enum Instruction {
    /// >
    IncrementPtr { by: usize },
    /// <
    DecrementPtr { by: usize },
    /// +
    IncrementByte { by: u8 },
    /// -
    DecrementByte { by: u8 },
    /// .
    Output,
    /// ,
    Input,
    /// [
    StartLoop(usize), // The end of the loop
    /// ]
    EndLoop(usize), // The start of the loop
    ///
    End,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::IncrementPtr { by } => write!(f, "> {}", by),
            Instruction::DecrementPtr { by } => write!(f, "< {}", by),
            Instruction::IncrementByte { by } => write!(f, "+ {}", by),
            Instruction::DecrementByte { by } => write!(f, "- {}", by),
            Instruction::Output => write!(f, ". output"),
            Instruction::Input => write!(f, ", input"),
            Instruction::StartLoop(end) => write!(f, "[ {}", end),
            Instruction::EndLoop(start) => write!(f, "] {}", start),
            Instruction::End => write!(f, "end"),
        }
    }
}

#[derive(Default)]
pub struct Instructions {
    instructions: Vec<Instruction>,
}

impl Instructions {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self { instructions }
    }

    pub fn get(&self, i: usize) -> Result<Instruction> {
        self.instructions
            .get(i)
            .cloned()
            .ok_or_else(|| Error::SegmentationFault("instruction not found".to_string()))
    }
}

impl Display for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let instructions = self
            .instructions
            .iter()
            .enumerate()
            .fold(String::new(), |acc, (line, element)| {
                format!("{}\n{} - {}", acc, line, element)
            });

        write!(f, "{}", instructions)
    }
}
