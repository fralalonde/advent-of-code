use std::io;
//use std::env;
use std::fs::File;
use anyhow::*;
use std::io::{BufReader, BufRead};

struct IntCodeMachine {
    cir: usize,
    pmodes: usize,
    memory: Vec<isize>,
    input: Box<Iterator<Item=isize>>,
    output: Vec<isize>,
}

enum PMode {
    Indirect = 0,
    Immediate = 1,
}

enum OpCode {
    Add = 01,
    Mult = 02,
    Read = 03,
    Write = 04,

    Halt = 99,
}

impl  IntCodeMachine {
    fn load(&mut self) -> isize {
        let addr = self.memory[self.cir] as usize;
        self.cir += 1;
        match self.pmode() {
            PMode::Immediate => self.memory[addr],
            PMode::Indirect => {
                let addr = self.memory[addr] as usize;
                self.memory[addr]
            }
        }
    }

    fn store(&mut self, value: isize){
        let addr = self.memory[self.cir] as usize;
        self.cir += 1;
        // "Parameters that an instruction writes to will never be in immediate mode."
        let addr = self.memory[addr] as usize;
        self.memory[addr] = value
    }

    pub fn init(&mut self, a: isize, b: isize) {
        let mut cir = 0;
        self.memory[1] = a;
        self.memory[2] = b;
    }

    fn decode(&mut self) -> OpCode {
        let op = self.memory[self.cir] as usize;
        self.cir += 1;
        let opcode = op % 100;
        self.pmodes = op / 100;
        opcode as OpCode
    }

    fn pmode(&mut self) -> PMode {
        let pmode = self.pmodes % 10;
        self.pmodes /= 10;
        pmode as PMode
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            let op = self.decode();
            match op.code {
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
                    self.output.push(value);
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
            .map(|x| x.clone().parse::<usize>().unwrap())
            .collect::<Vec<isize>>())
        .collect();

    let input = vec![1];
    let mut input_it = Box::new(input.iter());

    let mut machine = IntCodeMachine{
        memory,
        cir: 0,
        pmodes: 0,
        input: input_it,
        output: vec![],
    };

    machine.init(12,2);

    machine.run()?;

    // part A
    println!("A: {}", machine.memory[0]);

    Ok(())
}

#[cfg(test)]
mod test {

    #[test]
    fn tests() {
    }
}
