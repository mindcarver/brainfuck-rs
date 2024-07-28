// The middle indicates ADD ADD, ADD (3)

use std::{
    f32::consts::E,
    io::{Read, Write},
};

use opcode::Opcode;

mod opcode;

#[derive(Debug, PartialEq)]
pub enum IR {
    SHR(u32), // >>> === SHR(4)
    SHL(u32), // <<< === SHL(4)
    ADD(u8),
    SUB(u8),
    PUTCHAR,
    GETCHAR,
    JIZ(u32), // jump if zero
    JNZ(u32), // jump if not zero
}

pub struct Code {
    pub instrs: Vec<IR>,
}

impl Code {
    // 把opcode转成IR
    pub fn from_opcode(data: Vec<opcode::Opcode>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut instrs: Vec<IR> = Vec::new();
        let mut jstack: Vec<u32> = Vec::new();

        for e in data {
            match e {
                Opcode::SHR => match instrs.last_mut() {
                    Some(IR::SHR(x)) => {
                        *x += 1;
                    }
                    _ => {
                        instrs.push(IR::SHR(1));
                    }
                },
                Opcode::SHL => match instrs.last_mut() {
                    Some(IR::SHL(x)) => {
                        *x += 1;
                    }
                    _ => {
                        instrs.push(IR::SHL(1));
                    }
                },
                Opcode::ADD => match instrs.last_mut() {
                    Some(IR::ADD(x)) => {
                        *x += 1;
                    }
                    _ => {
                        instrs.push(IR::ADD(1));
                    }
                },
                Opcode::SUB => match instrs.last_mut() {
                    Some(IR::SUB(x)) => {
                        *x += 1;
                    }
                    _ => {
                        instrs.push(IR::SUB(1));
                    }
                },
                Opcode::PUTCHAR => instrs.push(IR::PUTCHAR),
                Opcode::GETCHAR => instrs.push(IR::GETCHAR),
                Opcode::LB => {
                    instrs.push(IR::JIZ((0)));
                    jstack.push((instrs.len() - 1) as u32);
                }
                Opcode::RB => {
                    let j = jstack.pop().ok_or("pop from empyt list")?;
                    instrs.push(IR::JNZ(j));
                    let instrs_len = instrs.len();
                    match &mut instrs[j as usize] {
                        IR::JIZ(x) => *x = (instrs_len - 1) as u32,
                        _ => unreachable!(),
                    }
                }
            }
        }

        Ok(Code { instrs })
    }
}

fn main(){}