use std::io::stdin;

use crate::error::{Error, Result};
use crate::symbol::{Instruction, Instructions};

struct Memory {
    memory: [u8; 30000],
}

impl Memory {
    fn new() -> Self {
        Self { memory: [0; 30000] }
    }

    fn add(&mut self, i: usize, value: u8) -> Result<u8> {
        self.memory
            .get_mut(i)
            .ok_or_else(|| Error::SegmentationFault("out of memory".to_string()))
            .and_then(|memory| {
                memory
                    .checked_add(value)
                    .ok_or(Error::Overflow)
                    .map(|result| {
                        (*memory) = result;
                        result
                    })
            })
    }

    fn decrement(&mut self, i: usize, value: u8) -> Result<u8> {
        self.memory
            .get_mut(i)
            .ok_or_else(|| Error::SegmentationFault("out of memory".to_string()))
            .and_then(|memory| {
                memory
                    .checked_sub(value)
                    .ok_or(Error::Overflow)
                    .map(|result| {
                        (*memory) = result;
                        result
                    })
            })
    }

    fn get(&self, i: &usize) -> Result<u8> {
        self.memory
            .get(*i)
            .ok_or_else(|| Error::SegmentationFault("out of memory".to_string()))
            .map(|result| *result)
    }

    fn set(&mut self, i: &usize, value: u8) -> Result<()> {
        self.memory
            .get_mut(*i)
            .ok_or_else(|| Error::SegmentationFault("out of memory".to_string()))
            .map(|memory| {
                (*memory) = value;
            })
    }
}

pub struct Interpreter {
    memory: Memory, // The memory of the brainfuck has 30k bytes
    pointer: usize, // The pointer of the memory
    i: usize,       // The current position in the program
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(), // The memory is initialized to 0
            pointer: 0,            // The pointer is initialized to the first cell of the memory: 0
            i: 0,
        }
    }

    pub fn exec(&mut self, instructions: Instructions) -> Result<()> {
        loop {
            // Run until there is no more instructions
            match instructions.get(self.i)? {
                Instruction::IncrementPtr { by } => {
                    self.pointer += by;
                    self.i += 1;
                }
                Instruction::DecrementPtr { by } => {
                    self.pointer -= by;
                    self.i += 1;
                }
                Instruction::IncrementByte { by } => {
                    self.memory.add(self.pointer, by)?;
                    self.i += 1;
                }
                Instruction::DecrementByte { by } => {
                    self.memory.decrement(self.pointer, by)?;
                    self.i += 1;
                }
                Instruction::Output => {
                    self.memory.get(&self.pointer).map(|result| {
                        print!("{}", result as char);
                    })?;
                    self.i += 1;
                }
                Instruction::Input => {
                    println!("input:");
                    let mut input_string = String::new();
                    let _ = stdin().read_line(&mut input_string);
                    let input_string = input_string.trim();
                    // Now the objective is to convert this to u8
                    match input_string.parse::<u8>() {
                        Ok(u8) => {
                            self.memory.set(&self.pointer, u8)?;
                        }
                        Err(_) => {
                            // We can try to convert to char
                            if !input_string.is_empty() {
                                // it means it's a char
                                self.memory.set(
                                    &self.pointer,
                                    input_string.chars().next().unwrap() as u8,
                                )?;
                            }
                        }
                    }
                    self.i += 1;
                }
                Instruction::StartLoop(end) => {
                    if self.memory.get(&self.pointer)? == 0 {
                        self.i = end + 1;
                    } else {
                        self.i += 1;
                    }
                }
                Instruction::EndLoop(start) => {
                    if self.memory.get(&self.pointer)? == 0 {
                        self.i += 1;
                    } else {
                        self.i = start;
                    }
                }
                Instruction::End => return Ok(()),
            }
        }
    }
}
