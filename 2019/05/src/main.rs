use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use strum::*;
use strum_macros::*;

#[repr(usize)]
#[derive(EnumIter, Debug, Clone, Copy)]
enum PMode {
    Indirect = 0,
    Immediate = 1,
}

impl From<usize> for PMode {
    fn from(a: usize) -> Self {
        for z in PMode::iter() {
            if z as usize == a {
                return z;
            }
        }
        panic!("no such pmode {}", a)
    }
}

#[repr(usize)]
#[derive(EnumIter, Debug, Clone, Copy)]
enum OpCode {
    Add = 01,
    Mult = 02,
    Read = 03,
    Write = 04,
    JumpIf = 05,
    JumpIfNot = 06,
    LessThan = 07,
    Equals = 08,

    Halt = 99,
}

impl From<usize> for OpCode {
    fn from(a: usize) -> Self {
        for z in OpCode::iter() {
            if z as usize == a {
                return z;
            }
        }
        panic!("no such opcode: {}", a)
    }
}

struct IntCodeMachine {
    cir: usize,
    pmodes: usize,
    memory: Vec<isize>,
    input: Box<dyn Iterator<Item = isize>>,
}

impl IntCodeMachine {
    fn load(&mut self) -> isize {
        let param = self.memory[self.cir];
        self.cir += 1;
        match self.pmode() {
            PMode::Immediate => param,
            PMode::Indirect => self.memory[param as usize],
        }
    }

    fn store(&mut self, value: isize) {
        let ptr = self.memory[self.cir] as usize;
        self.cir += 1;
        // "Parameters that an instruction writes to will never be in immediate mode."
        self.memory[ptr] = value
    }

    fn decode(&mut self) -> OpCode {
        let op = self.memory[self.cir] as usize;
        self.cir += 1;
        let opcode = op % 100;
        self.pmodes = op / 100;
        OpCode::from(opcode)
    }

    fn pmode(&mut self) -> PMode {
        let pmode = self.pmodes % 10;
        self.pmodes /= 10;
        PMode::from(pmode)
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            let opcode = self.decode();
//            println!("opcode {:?}", opcode);
            match opcode {
                OpCode::Add => {
                    let a = self.load();
                    let b = self.load();
                    self.store(a + b);
                }
                OpCode::Mult => {
                    let a = self.load();
                    let b = self.load();
                    self.store(a * b);
                }
                OpCode::Read => {
                    let value = self.input.next().unwrap();
                    self.store(value);
                }
                OpCode::Write => {
                    let value = self.load();
                    println!("output {}", value);
                }
                OpCode::JumpIf => {
                    let a = self.load();
                    let b = self.load();
                    if a != 0 {
                        self.cir = b as usize
                    }
                }
                OpCode::JumpIfNot => {
                    let a = self.load();
                    let b = self.load();
                    if a == 0 {
                        self.cir = b as usize
                    }
                }
                OpCode::LessThan => {
                    let a = self.load();
                    let b = self.load();
                    self.store(if a < b { 1 } else { 0 });
                }
                OpCode::Equals => {
                    let a = self.load();
                    let b = self.load();
                    self.store(if a == b { 1 } else { 0 });
                }
                OpCode::Halt => break,
            };
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let f = File::open("05/input")?;
    let file = BufReader::new(&f);
    let memory: Vec<isize> = file
        .lines()
        .into_iter()
        .filter_map(|x| x.ok())
        .flat_map(|s| {
            s.split(",")
                .map(|x| x.clone().parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect();

    // part A

    let input: Vec<isize> = vec![1];
    let input_it = Box::new(input.into_iter());

    let mut machine = IntCodeMachine {
        memory: memory.clone(),
        cir: 0,
        pmodes: 0,
        input: input_it,
    };

    machine.run()?;

    // part B

    let input: Vec<isize> = vec![5];
    let input_it = Box::new(input.into_iter());

    let mut machine = IntCodeMachine {
        memory: memory.clone(),
        cir: 0,
        pmodes: 0,
        input: input_it,
    };

    machine.run()?;

    Ok(())
}

#[cfg(test)]
mod test {

    #[test]
    fn tests() {}
}
