use std::io::{Read, Write};
use std::path::Path;
use std::{fs::File, io};

use crate::constants::{ADD, ADJ_BASE, EQ, IN, JMP_FALSE, JMP_TRUE, LESS, MULT, OUT, RET};
use crate::param::ParamMode;

const MEM_SIZE: usize = 30000;

#[derive(Debug)]
pub struct VM {
    memory: [i64; MEM_SIZE],
    pc: usize,
    rel_base: i64,
    buffer: String,
    buffer_read: usize,
    ascii: bool,
}

impl Default for VM {
    fn default() -> Self {
        Self {
            memory: [0; MEM_SIZE],
            pc: 0,
            rel_base: 0,
            buffer: String::new(),
            buffer_read: 0,
            ascii: false,
        }
    }
}

impl VM {
    pub fn new(ascii: bool) -> Self {
        Self {
            ascii,
            ..Self::default()
        }
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

        buffer.chunks(8).enumerate().for_each(|(index, chunk)| {
            let chunk: [u8; 8] = chunk.try_into().unwrap();
            self.memory[index] = i64::from_le_bytes(chunk);
        });
    }

    fn get_param_value(&self, param: &ParamMode) -> i64 {
        match param {
            ParamMode::Position(pos) => self.memory[*pos],
            ParamMode::Immediate(val) => *val,
            ParamMode::Relative(pos) => self.memory[(*pos + self.rel_base) as usize],
        }
    }

    #[allow(dead_code)]
    pub fn get_first_value(&self) -> &i64 {
        self.memory.first().unwrap()
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

                    let value = if self.ascii {
                        if self.buffer_read >= self.buffer.len() {
                            print!("> ");
                            io::stdout().flush().unwrap();

                            let mut buffer = String::new();
                            io::stdin().read_line(&mut buffer).unwrap();
                            self.buffer = buffer.clone();
                            self.buffer_read = 0;
                        }

                        let value = self.buffer.chars().nth(self.buffer_read).unwrap() as i64;
                        self.buffer_read += 1;
                        value
                    } else {
                        print!("> ");
                        io::stdout().flush().unwrap();
                        let mut buffer = String::new();
                        io::stdin().read_line(&mut buffer).unwrap();
                        let buffer = self.buffer.trim();
                        let value: i64 = buffer.parse().unwrap();
                        value
                    };

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

                    if self.ascii {
                        print!("{}", char::from_u32(a as u32).unwrap());
                    } else {
                        println!("{}", a);
                    }

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
