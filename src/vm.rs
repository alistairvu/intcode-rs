use std::io::{Read, Write};
use std::path::Path;
use std::{fs::File, io};

use crate::constants::{ADD, EQ, IN, JMP_FALSE, JMP_TRUE, LESS, MULT, OUT, RET};
use crate::param::ParamMode;

#[derive(Default, Debug)]
pub struct VM {
    memory: Vec<i32>,
    pc: usize,
}

impl VM {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn read(&mut self, filename: &str) {
        let path = Path::new(&filename);
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(why) => panic!("Couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        let mut buffer = vec![];

        let _file_size = file
            .read_to_end(&mut buffer)
            .unwrap_or_else(|e| panic!("Couldn't read {}: {}", display, e));

        let memory_vec = buffer.chunks(4).map(|chunk| {
            let value = (chunk[3] as i32) << 24
                | (chunk[2] as i32) << 16
                | (chunk[1] as i32) << 8
                | chunk[0] as i32;

            if value > 0x40000000 {
                value - 0x40000000
            } else {
                value
            }
        });

        for value in memory_vec {
            self.memory.push(value);
        }
    }

    fn get_param_value(&self, param: &ParamMode) -> i32 {
        match param {
            ParamMode::Position(pos) => self.memory[*pos],
            ParamMode::Immediate(val) => *val,
        }
    }

    pub fn run(&mut self) {
        while self.pc < self.memory.len() {
            match self.memory[self.pc] % 100 {
                ADD => {
                    let params = ParamMode::get_params(&self.memory[self.pc..self.pc + 4]);

                    let a = self.get_param_value(&params[0]);
                    let b = self.get_param_value(&params[1]);

                    if let ParamMode::Position(pos) = params[2] {
                        self.memory[pos] = a + b;
                    }

                    self.pc += 4;
                }

                MULT => {
                    let params = ParamMode::get_params(&self.memory[self.pc..self.pc + 4]);

                    let a = self.get_param_value(&params[0]);
                    let b = self.get_param_value(&params[1]);

                    if let ParamMode::Position(pos) = params[2] {
                        self.memory[pos] = a * b;
                    }

                    self.pc += 4;
                }

                IN => {
                    let a_pos = self.memory[self.pc + 1];

                    print!("> ");
                    io::stdout().flush().unwrap();

                    let mut buffer = String::new();
                    io::stdin().read_line(&mut buffer).unwrap();

                    let buffer = buffer.trim();
                    let value: i32 = buffer.parse().unwrap();

                    self.memory[a_pos as usize] = value;

                    self.pc += 2;
                }

                OUT => {
                    let params = ParamMode::get_params(&self.memory[self.pc..self.pc + 2]);
                    let a = self.get_param_value(&params[0]);

                    println!("{}", a);

                    self.pc += 2;
                }

                JMP_TRUE => {
                    let params = ParamMode::get_params(&self.memory[self.pc..self.pc + 3]);
                    let a = self.get_param_value(&params[0]);
                    let b = self.get_param_value(&params[1]);

                    if a != 0 {
                        self.pc = b as usize;
                    } else {
                        self.pc += 3;
                    }
                }

                JMP_FALSE => {
                    let params = ParamMode::get_params(&self.memory[self.pc..self.pc + 3]);
                    let a = self.get_param_value(&params[0]);
                    let b = self.get_param_value(&params[1]);

                    if a == 0 {
                        self.pc = b as usize;
                    } else {
                        self.pc += 3;
                    }
                }

                LESS => {
                    let params = ParamMode::get_params(&self.memory[self.pc..self.pc + 4]);

                    let a = self.get_param_value(&params[0]);
                    let b = self.get_param_value(&params[1]);

                    if let ParamMode::Position(pos) = params[2] {
                        self.memory[pos] = {
                            if a < b {
                                1
                            } else {
                                0
                            }
                        };
                    }

                    self.pc += 4;
                }

                EQ => {
                    let params = ParamMode::get_params(&self.memory[self.pc..self.pc + 4]);

                    let a = self.get_param_value(&params[0]);
                    let b = self.get_param_value(&params[1]);

                    if let ParamMode::Position(pos) = params[2] {
                        self.memory[pos] = {
                            if a == b {
                                1
                            } else {
                                0
                            }
                        };
                    }

                    self.pc += 4;
                }

                RET => {
                    return;
                }

                _ => {
                    println!("[{}] Invalid opcode: {}", self.pc, self.memory[self.pc]);
                    return;
                }
            }
        }
    }
}
