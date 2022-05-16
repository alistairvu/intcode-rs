use std::io::{Read, Write};
use std::path::Path;
use std::{fs::File, io};

use crate::constants::{ADD, ADJ_BASE, EQ, IN, JMP_FALSE, JMP_TRUE, LESS, MULT, OUT, RET};
use crate::param::ParamMode;

const MIN_MEM_SIZE: usize = 30000;

#[derive(Default, Debug)]
pub struct VM {
    memory: Vec<i64>,
    pc: usize,
    rel_base: i64,
    // buffer: String,
    // buffer_read: usize,
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

        let memory_vec = buffer.chunks(8).map(|chunk| {
            let chunk: [u8; 8] = chunk.try_into().unwrap();
            // let value =
            i64::from_le_bytes(chunk)
        });

        for value in memory_vec {
            self.memory.push(value);
        }

        while self.memory.len() < MIN_MEM_SIZE {
            self.memory.push(0);
        }
    }

    fn get_param_value(&self, param: &ParamMode) -> i64 {
        match param {
            ParamMode::Position(pos) => self.memory[*pos],
            ParamMode::Immediate(val) => *val,
            ParamMode::Relative(pos) => self.memory[(*pos + self.rel_base) as usize],
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
                    } else if let ParamMode::Relative(pos) = params[2] {
                        self.memory[(pos + self.rel_base) as usize] = a + b;
                    }

                    self.pc += 4;
                }

                MULT => {
                    let params = ParamMode::get_params(&self.memory[self.pc..self.pc + 4]);

                    let a = self.get_param_value(&params[0]);
                    let b = self.get_param_value(&params[1]);

                    if let ParamMode::Position(pos) = params[2] {
                        self.memory[pos] = a * b;
                    } else if let ParamMode::Relative(pos) = params[2] {
                        self.memory[(pos + self.rel_base) as usize] = a * b;
                    }

                    self.pc += 4;
                }

                IN => {
                    let params = ParamMode::get_params(&self.memory[self.pc..self.pc + 2]);

                    // if self.buffer_read >= self.buffer.len() {
                    print!("> ");
                    io::stdout().flush().unwrap();

                    let mut buffer = String::new();
                    io::stdin().read_line(&mut buffer).unwrap();
                    // self.buffer = buffer.clone();
                    // self.buffer_read = 0;
                    // }

                    let buffer = buffer.trim();
                    let value: i64 = buffer.parse().unwrap();

                    // self.buffer_read += 1;

                    if let ParamMode::Position(pos) = params[0] {
                        self.memory[pos as usize] = value;
                    } else if let ParamMode::Relative(pos) = params[0] {
                        self.memory[(pos + self.rel_base) as usize] = value;
                    }

                    self.pc += 2;
                }

                OUT => {
                    let params = ParamMode::get_params(&self.memory[self.pc..self.pc + 2]);
                    let a = self.get_param_value(&params[0]);

                    // print!("{}", char::from_u32(a as u32).unwrap());
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

                    let result = if a < b { 1 } else { 0 };

                    if let ParamMode::Position(pos) = params[2] {
                        self.memory[pos] = result;
                    } else if let ParamMode::Relative(pos) = params[2] {
                        self.memory[(pos + self.rel_base) as usize] = result;
                    }

                    self.pc += 4;
                }

                EQ => {
                    let params = ParamMode::get_params(&self.memory[self.pc..self.pc + 4]);

                    let a = self.get_param_value(&params[0]);
                    let b = self.get_param_value(&params[1]);

                    let result = if a == b { 1 } else { 0 };

                    if let ParamMode::Position(pos) = params[2] {
                        self.memory[pos] = result;
                    } else if let ParamMode::Relative(pos) = params[2] {
                        self.memory[(pos + self.rel_base) as usize] = result;
                    }

                    self.pc += 4;
                }

                ADJ_BASE => {
                    let params = ParamMode::get_params(&self.memory[self.pc..self.pc + 2]);

                    let a = self.get_param_value(&params[0]);
                    self.rel_base += a;
                    self.pc += 2;
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
