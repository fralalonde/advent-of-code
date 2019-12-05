use std::io;
//use std::env;
use std::fs::File;
use anyhow::*;
use std::io::{BufReader, BufRead};
use strum::*;
use strum_macros::*;

#[repr(usize)]
#[derive(EnumIter,Debug, Clone, Copy)]
enum PMode {
    Indirect = 0,
    Immediate = 1,
}

impl From<usize> for PMode {
    fn from(a: usize) -> Self {
        for z in PMode::iter() {
            if z as usize == a {
                return z
            }
        }
        panic!("no such pmode {}", a)
    }
}

#[repr(usize)]
#[derive(EnumIter,Debug, Clone, Copy)]
enum OpCode {
    Add = 01,
    Mult = 02,
    Read = 03,
    Write = 04,

    Halt = 99,
}

impl From<usize> for OpCode {
    fn from(a: usize) -> Self {
        for z in OpCode::iter() {
            if z as usize == a {
                return z
            }
        }
        panic!("no such opcode: {}", a)
    }
}


struct IntCodeMachine {
    cir: usize,
    pmodes: usize,
    memory: Vec<isize>,
    input: Box<Iterator<Item=isize>>,
    output: Vec<isize>,
}

impl  IntCodeMachine {
    fn load(&mut self) -> isize {
        let param = self.memory[self.cir];
        self.cir += 1;
        match self.pmode() {
            PMode::Immediate => param,
            PMode::Indirect => self.memory[param as usize]
        }
    }

    fn store(&mut self, value: isize){
        let ptr = self.memory[self.cir] as usize;
        self.cir += 1;
        // "Parameters that an instruction writes to will never be in immediate mode."
        self.memory[ptr] = value
    }

    pub fn init(&mut self, a: isize, b: isize) {
        self.cir = 0;
        self.memory[1] = a;
        self.memory[2] = b;
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
            match opcode {
                OpCode::Add => {
                    let a =  self.load();
                    let b =  self.load();
                    self.store(a + b);
                }
                OpCode::Mult => {
                    let a =  self.load();
                    let b =  self.load();
                    self.store(a * b);
                }
                OpCode::Read => {
                    let value = self.input.next().unwrap();
                    self.store(value);
                }
                OpCode::Write => {
                    let value = self.load();
                    println!("{}", value);
//                    self.output.push(value);
                }
                OpCode::Halt => {
                    break
                }
                _ => return Err(anyhow!("Mfuck")),
            };
        }
        Ok(())
    }

}


fn main() -> Result<()> {
    let f = File::open("05/input")?;
    let file = BufReader::new(&f);
    let mut memory: Vec<isize> = file.lines().into_iter()
        .filter_map(|x| x.ok())
        .flat_map(|s| s.split(",")
            .map(|x| x.clone().parse::<isize>().unwrap())
            .collect::<Vec<isize>>())
        .collect();

    let input: Vec<isize> = vec![1];
    {
        let mut input_it = Box::new(input.into_iter());

        let mut machine = IntCodeMachine {
            memory,
            cir: 0,
            pmodes: 0,
            input: input_it,
            output: vec![],
        };

        machine.run()?;

        // part A
        println!("A: {}", machine.memory[0]);
    }
    Ok(())
}

#[cfg(test)]
mod test {

    #[test]
    fn tests() {
    }
}
