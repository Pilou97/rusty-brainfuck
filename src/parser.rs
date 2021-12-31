use std::collections::HashMap;

use crate::error::{Error, Result};
use crate::symbol::{Instruction, Instructions};

pub trait Parser {
    fn parse(&self) -> Result<Instructions>;
}

/// Hashmap of jump (from, to)
type Jumps = HashMap<usize, usize>;
fn compute_jump(instructions: &[char]) -> Result<(Jumps, Jumps)> {
    let mut stack = Vec::new();
    instructions.iter().enumerate().fold(
        Ok((HashMap::new(), HashMap::new())),
        |jumps, (index, &character)| match jumps {
            Err(reason) => Err(reason),
            Ok((mut opening, mut closing)) => match character {
                '[' => {
                    stack.push(('[', index));
                    Ok((opening, closing))
                }
                ']' => {
                    if matches!(stack.last(), Some(('[', _))) {
                        let opening_bracket = stack.remove(stack.len() - 1);
                        opening.insert(opening_bracket.1, index);
                        closing.insert(index, opening_bracket.1);
                        Ok((opening, closing))
                    } else {
                        Err(Error::WrongParenthesis { instruction: index })
                    }
                }
                _ => Ok((opening, closing)),
            },
        },
    )
}

/// Just parse a string
pub struct StringParser {
    raw: String,
}

impl StringParser {
    pub fn new(string: String) -> Self {
        Self { raw: string }
    }
}

impl Parser for StringParser {
    fn parse(&self) -> Result<Instructions> {
        let instructions = self
            .raw
            .chars()
            .into_iter()
            .filter(|char| matches!(char, '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']'))
            .collect::<Vec<char>>();

        let (opening, closing) = compute_jump(&instructions)?;

        let mut instructions = instructions.iter().enumerate().fold(
            Ok(Vec::<Instruction>::new()),
            |acc, (i, char)| match acc {
                Err(err) => Err(err),
                Ok(mut acc) => {
                    match char {
                        '>' => acc.push(Instruction::IncrementPtr { by: 1 }),
                        '<' => acc.push(Instruction::DecrementPtr { by: 1 }),
                        '+' => acc.push(Instruction::IncrementByte { by: 1 }),
                        '-' => acc.push(Instruction::DecrementByte { by: 1 }),
                        '.' => acc.push(Instruction::Output),
                        ',' => acc.push(Instruction::Input),
                        '[' => {
                            let closing = *opening
                                .get(&i)
                                .ok_or(Error::WrongParenthesis { instruction: i })?;
                            acc.push(Instruction::StartLoop(closing));
                        }
                        ']' => {
                            let starting = *closing
                                .get(&i)
                                .ok_or(Error::WrongParenthesis { instruction: i })?;
                            acc.push(Instruction::EndLoop(starting))
                        }
                        _ => {}
                    };
                    Ok(acc)
                }
            },
        )?;

        instructions.push(Instruction::End);

        Ok(Instructions::new(instructions))
    }
}
