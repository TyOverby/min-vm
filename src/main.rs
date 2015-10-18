use std::cmp::{Ord, Ordering};

mod util;

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    ExplicitCrash,
    OutOfInstructions,
    HitZero,
    BadRegister(u8),
    UnknownType,
    AssignToConstant,
    UnknownInstruction{instr: u8, at: u8}
}

const TY_CONST:u8 = 0;
const TY_REG  :u8 = 1;
const TY_MEM_C:u8 = 2;
const TY_MEM_R:u8 = 3;

const CRASH:u8 = 0;
const PRINT:u8 = 1;
const ADD:u8 = 2;
const SUB:u8 = 3;
const MUL:u8 = 4;
const DIV:u8 = 5;
const MOD:u8 = 6;
const JMP:u8 = 7;
const CMP:u8 = 8;
const AND:u8 = 9;
const OR:u8 = 10;
const IF:u8 = 11;
const CALL:u8 = 12;
const MOVE:u8 = 13;


pub struct Machine {
    mem: [u8; 256],
    reg: [u8; 8],
    ip: u8
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            mem: [0; 256],
            reg: [0; 8],
            ip: 0
        }
    }

    pub fn load(&mut self, instrs: &[u8]) {
        util::copy(&mut self.mem, instrs);
    }

    fn get(&self, typ: u8, val: u8) -> Result<u8, Error> {
        match typ {
            TY_CONST => Ok(val),
            TY_REG   => {
                if val < 7 {
                    Ok(self.reg[val as usize])
                } else {
                    Err(Error::BadRegister(val))
                }
            }
            TY_MEM_C => {
                Ok(self.mem[val as usize])
            }
            TY_MEM_R => {
                if val < 7 {
                    Ok(self.mem[self.reg[val as usize] as usize])
                } else {
                    Err(Error::BadRegister(val))
                }
            }
            _ => Err(Error::UnknownType)
        }
    }

    fn set(&mut self, typ: u8, data: u8, value: u8) -> Result<(), Error> {
        match typ {
            TY_CONST => Err(Error::AssignToConstant),
            TY_REG   => {
                if data < 7 {
                    self.reg[data as usize] = value;
                    Ok(())
                } else {
                    Err(Error::BadRegister(data))
                }
            }
            TY_MEM_C => {
                self.mem[data as usize] = value;
                Ok(())
            }
            TY_MEM_R => {
                if data < 7 {
                    self.mem[self.reg[data as usize] as usize] = value;
                    Ok(())
                } else {
                    Err(Error::BadRegister(data))
                }
            }
            _ => Err(Error::UnknownType)
        }

    }

    fn binop<F>(&mut self, operands: &[u8], f: F) -> Result<(), Error>
        where F: FnOnce(u8, u8) -> u8 {
            let o1 = try!(self.get(operands[0], operands[1]));
            let o2 = try!(self.get(operands[2], operands[3]));
            self.set(operands[2], operands[3], f(o1, o2))
        }

    pub fn step(&mut self) -> Result<(), Error> {
        let mut instr = [0; 5];
        util::copy(&mut instr[..], &self.mem[self.ip as usize .. self.ip as usize + 5]);
        self.ip += 5;
        let op = instr[0];
        let operands = &instr[1 .. 5];

        match op {
            CRASH => return Err(Error::ExplicitCrash),
            PRINT => println!("{}", try!(self.get(operands[0], operands[1])) as char),
            ADD => try!(self.binop(operands, |s, d| d + s)),
            SUB => try!(self.binop(operands, |s, d| d - s)),
            MUL => try!(self.binop(operands, |s, d| d * s)),
            DIV => try!(self.binop(operands, |s, d| d / s)),
            MOD => try!(self.binop(operands, |s, d| d % s)),
            JMP => self.ip = try!(self.get(operands[0], operands[1])),
            CMP => try!(self.binop(operands, |s, d|
                match d.cmp(&s) {
                    Ordering::Greater => 255,
                    Ordering::Less => 1,
                    Ordering::Equal => 0
            })),
            AND => try!(self.binop(operands, |s, d| s & d)),
            OR => try!(self.binop(operands, |s, d| s | d)),
            IF => {

            }
            CALL => {

            }
            MOVE => try!(self.binop(operands, |s, _d| s)),
            other => return Err(Error::UnknownInstruction{ instr: other, at: self.ip - 5 })
        }
        Ok(())
    }
}

#[test]
fn test_crash() {
    let mut machine = Machine::new();
    assert_eq!(machine.step(), Err(Error::ExplicitCrash));
}

#[test]
fn set_to_const() {
    let mut machine = Machine::new();
    machine.load(&[MOVE, TY_CONST, 5, TY_CONST, 10]);
    assert_eq!(machine.step(), Err(Error::AssignToConstant));
}

#[test]
fn set_to_reg() {
    let mut machine = Machine::new();
    machine.load(&[MOVE, TY_CONST, 5, TY_REG, 15]);
    assert_eq!(machine.step(), Err(Error::BadRegister(15)));

    let mut machine = Machine::new();
    machine.load(&[MOVE, TY_CONST, 5, TY_REG, 0]);
    assert!(machine.step().is_ok());
    assert_eq!(machine.reg[0], 5);
}

#[test]
fn set_to_mem_const() {
    let mut machine = Machine::new();
    machine.load(&[MOVE, TY_CONST, 5, TY_MEM_C, 15]);
    assert!(machine.step().is_ok());
    assert_eq!(machine.mem[15], 5);
}

#[test]
fn set_to_mem_reg() {
    let mut machine = Machine::new();
    machine.load(&[MOVE, TY_CONST, 15, TY_REG, 0, MOVE, TY_CONST, 5, TY_MEM_R, 0]);
    assert!(machine.step().is_ok());
    assert!(machine.step().is_ok());
    assert_eq!(machine.mem[15], 5);
}
