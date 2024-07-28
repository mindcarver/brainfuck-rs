mod opcode;
use std::{
    io::{Read, Write},
    vec,
};

use opcode::{Code, Opcode};

struct Interpreter {
    stack: Vec<u8>,
}

impl Interpreter {
    fn new() -> Self {
        Self { stack: vec![0, 1] }
    }

    fn run(&mut self, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let code = Code::from(data)?;
        let code_len = code.instrs.len();
        let mut pc = 0; // program counter
        let sp = 0; // stack pointer

        loop {
            if pc >= code_len {
                break;
            }
            match code.instrs[pc] {
                Opcode::SHR => {
                    pc += 1;
                    if pc == self.stack.len() {
                        self.stack.push(0);
                    }
                }
                Opcode::SHL => {
                    if pc != 0 {
                        pc -= 1;
                    }
                }
                Opcode::ADD => {
                    self.stack[sp] = self.stack[sp].overflowing_add(1).0;
                }
                Opcode::SUB => {
                    self.stack[sp] = self.stack[sp].overflowing_sub(1).0;
                }
                Opcode::PUTCHAR => {
                    std::io::stdout().write_all(&[self.stack[pc]])?;
                }
                Opcode::GETCHAR => {
                    let mut buf: Vec<u8> = vec![0, 1];
                    std::io::stdin().read_exact(&mut buf)?;
                    self.stack[sp] = buf[0];
                }
                Opcode::LB => {
                    if self.stack[pc] == 0x00 {
                        pc = code.jtables[&pc];
                    }
                }
                Opcode::RB => {
                    if self.stack[pc] == 0x00 {
                        pc = code.jtables[&pc];
                    }
                }
            }
            pc += 1;
        }

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let data = std::fs::read(args[1].clone())?;

    let mut interpreter = Interpreter::new();
    let _ = interpreter.run(data);

    Ok(())
}
