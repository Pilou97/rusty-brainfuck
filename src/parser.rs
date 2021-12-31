use std::collections::HashMap;

use crate::symbol::{Instruction, Instructions};

pub trait Parser {
    fn parse(&self) -> Result<Instructions, String>;
}

/// Hashmap of jump (from, to)

type Jumps = HashMap<usize, usize>;

fn compute_jump(instructions: &[char]) -> Result<(Jumps, Jumps), String> {
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
                        Err("Wrong Parenthesis".to_string())
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
    fn parse(&self) -> Result<Instructions, String> {
        let instructions = self
            .raw
            .chars()
            .into_iter()
            .filter(|char| matches!(char, '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']'))
            .collect::<Vec<char>>();

        let (opening, closing) = compute_jump(&instructions)?;

        let instructions = instructions
            .iter()
            .enumerate()
            .filter_map(|(i, char)| match char {
                '>' => Some(Instruction::IncrementPtr { by: 1 }),
                '<' => Some(Instruction::DecrementPtr { by: 1 }),
                '+' => Some(Instruction::IncrementByte { by: 1 }),
                '-' => Some(Instruction::DecrementByte { by: 1 }),
                '.' => Some(Instruction::Output),
                ',' => Some(Instruction::Input),
                '[' => Some(Instruction::StartLoop(
                    *opening.get(&i).expect(&format!("line {}", i)),
                )),
                ']' => Some(Instruction::EndLoop(*closing.get(&i).unwrap())),
                _ => None,
            })
            .collect();

        Ok(Instructions::new(instructions))
    }
}
