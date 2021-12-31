use std::io::stdin;

use crate::symbol::{Instruction, Instructions};

pub struct Interpreter {
    memory: [u8; 30000], // The memory of the brainfuck has 30k bytes
    pointer: usize,      // The pointer of the memory
    i: usize,            // The current position in the program
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            memory: [0; 30000], // The memory is initialized to 0
            pointer: 0,         // The pointer is initialized to the first cell of the memory: 0
            i: 0,
        }
    }

    pub fn exec(&mut self, instructions: Instructions) {
        loop {
            // Run until there is no more instructions
            let instruction = instructions.get(self.i);
            if let Some(instruction) = instruction {
                match instruction {
                    Instruction::IncrementPtr { by } => {
                        self.pointer += by;
                        self.i += 1;
                    }
                    Instruction::DecrementPtr { by } => {
                        self.pointer -= by;
                        self.i += 1;
                    }
                    Instruction::IncrementByte { by } => match self.memory.get_mut(self.pointer) {
                        None => return,
                        Some(memory) => {
                            (*memory) += by;
                            self.i += 1;
                        }
                    },
                    Instruction::DecrementByte { by } => match self.memory.get_mut(self.pointer) {
                        None => return,
                        Some(memory) => {
                            (*memory) -= by;
                            self.i += 1;
                        }
                    },
                    Instruction::Output => match self.memory.get(self.pointer) {
                        None => return,
                        Some(&character) => {
                            print!("{}", character as char);
                            self.i += 1;
                        }
                    },
                    Instruction::Input => {
                        println!("input:");
                        let mut input_string = String::new();
                        let _ = stdin().read_line(&mut input_string);
                        let input_string = input_string.trim();
                        // Now the objective is to convert this to u8
                        match input_string.parse::<u8>() {
                            Ok(u8) => match self.memory.get_mut(self.pointer) {
                                None => return,
                                Some(memory) => {
                                    (*memory) = u8;
                                }
                            },
                            Err(_) => {
                                // We can try to convert to char
                                if !input_string.is_empty() {
                                    // it means it's a char
                                    match self.memory.get_mut(self.pointer) {
                                        None => return,
                                        Some(memory) => {
                                            (*memory) = input_string.chars().next().unwrap() as u8;
                                        }
                                    }
                                }
                            }
                        }

                        self.i += 1;
                    }
                    Instruction::StartLoop(end) => {
                        if let Some(0) = self.memory.get(self.pointer) {
                            self.i = end + 1;
                        } else {
                            self.i += 1;
                        }
                    }
                    Instruction::EndLoop(start) => {
                        if let Some(0) = self.memory.get(self.pointer) {
                            self.i += 1;
                        } else {
                            self.i = start;
                        }
                    }
                }
            } else {
                return;
            }
        }
    }
}
