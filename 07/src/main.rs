use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use strum::*;
use strum_macros::*;
use futures_channel::mpsc::*;
use futures::{StreamExt, SinkExt};
use futures_executor::LocalPool;

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
    input: Receiver<isize>,
    output: Sender<isize>,
}

impl IntCodeMachine {
    fn new(memory: Vec<isize>, input: Receiver<isize>, output: Sender<isize>) -> Self {
        IntCodeMachine {
            cir: 0,
            pmodes: 0,
            memory,
            input,
            output
        }
    }

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

    fn log(&self, _msg: &str) {
//        println!("{}: {}", self.name, msg)
    }

    fn pmode(&mut self) -> PMode {
        let pmode = self.pmodes % 10;
        self.pmodes /= 10;
        PMode::from(pmode)
    }

    pub async fn run(mut self) -> Result<IntCodeMachine> {
        self.log("Started");
        loop {
            self.log("Decoding");
            let opcode = self.decode();

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
                    self.log("Reading");
                    let value = self.input.next().await.expect("Read");
                    self.store(value);
                }
                OpCode::Write => {
                    self.log("Writing");
                    let value = self.load();
                    self.output.send(value).await.expect("Write");
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
            }
        }
        Ok(self)
    }
}

fn main() -> Result<()> {

    let f = File::open("07/input").unwrap();
    let file = BufReader::new(&f);
    let mut max = 0;
    let program: Vec<isize> = file
        .lines()
        .into_iter()
        .filter_map(|x| x.ok())
        .flat_map(|s| {
            s.split(",")
                .map(|x| x.clone().parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect();

    let mem1  = program.clone();

    let part_one = async move {
        println!("Running part A");
        for a in 0..5 {
            for b in 0..5 {
                if a == b {continue}
                for c in 0..5 {
                    if a == c  || b == c {continue}
                    for d in 0..5 {
                        if a == d  || b == d || c == d {continue}
                        for e in 0..5 {
                            if a == e  || b == e || c == e || d == e {continue}
                            let phases = vec![a, b, c, d, e];
                            let mut phase = phases.clone().into_iter();
                            let (mut input, recv) = channel::<isize>(2);
                            input.send(phase.next().unwrap()).await.expect("Phase Write");
                            input.send(0).await.expect("Init Level Write");

                            let mut recv: Option<Receiver<isize>> = Some(recv);

                            let mut machines = vec![];
                            for v in 0..5 {
                                let (mut out, recv_next) = channel(2);
                                if v < 4 {
                                    out.send(phase.next().unwrap()).await.expect("Phase Write");
                                }
                                machines.push(IntCodeMachine::new(mem1.clone(), recv.take().unwrap(), out));
                                recv = Some(recv_next)
                            }

                            let mut runs = vec![];
                            for m in machines.into_iter() {
                                runs.push(m.run())
                            }
                            let _results: Vec<Result<IntCodeMachine, Error>> = futures::future::join_all(runs).await.drain(..).collect();

                            if let Some(mut recv) = recv {
                                let output = recv.next().await.unwrap();
                                if output > max {
                                    max = output
                                }
                            }
                        }
                    }
                }
            }
        }
        println!("max output A {}", max);
    };

    let mem2  = program.clone();
    let mut pool = LocalPool::new();
    pool.run_until(part_one);

    let part_two = async move {
        println!("Running part B");
        for a in 5..10 {
            for b in 5..10 {
                if a == b {continue}
                for c in 5..10 {
                    if a == c  || b == c {continue}
                    for d in 5..10 {
                        if a == d  || b == d || c == d {continue}
                        for e in 5..10 {
                            if a == e  || b == e || c == e || d == e {continue}
                            let phases = vec![a, b, c, d, e];
                            let mut phase = phases.clone().into_iter();
                            let (_input, recv) = channel::<isize>(2);
                            let phase_zero = phase.next().unwrap();

                            let mut recv: Option<Receiver<isize>> = Some(recv);

                            let mut machines = vec![];
                            for v in 0..5 {
                                let (mut out, recv_next) = channel(2);
                                if v < 4 {
                                    out.send(phase.next().unwrap()).await.expect("Phase Write");
                                }
                                machines.push(IntCodeMachine::new(mem2.clone(), recv.take().unwrap(), out));
                                recv = Some(recv_next)
                            }

                            machines[0].input = recv.unwrap();
                            machines[4].output.send(phase_zero).await.unwrap();
                            machines[4].output.send(0).await.unwrap();

                            let mut runs = vec![];
                            for m in machines.into_iter() {
                                runs.push(m.run())
                            }
                            let mut results: Vec<Result<IntCodeMachine, Error>> = futures::future::join_all(runs).await.drain(..).collect();

                            if let Ok(r) = results.get_mut(0).unwrap() {
                                let output = r.input.next().await.unwrap();
                                if output > max {
                                    max = output
                                }
                            }
                        }
                    }
                }
            }
        }
        println!("max output B {}", max);
    };
    pool.run_until(part_two);

    Ok(())
}

#[cfg(test)]
mod test {

    #[test]
    fn tests() {

    }
}
