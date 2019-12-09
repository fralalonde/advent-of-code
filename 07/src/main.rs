use std::io;
//use std::env;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use strum::*;
use strum_macros::*;
use std::collections::VecDeque;
use std::rc::Rc;
use futures_channel::mpsc::*;
use futures::stream::Stream;
use async_std::*;
use async_task::*;
use std::cell::RefCell;
use futures::task::Poll;
use futures::future::PollFn;
use futures::{StreamExt, SinkExt};

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

struct RecvPoll {
    input: Receiver<isize>
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
//        println!("Decode");
        OpCode::from(opcode)
    }

    fn pmode(&mut self) -> PMode {
        let pmode = self.pmodes % 10;
        self.pmodes /= 10;
        PMode::from(pmode)
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            print!("Decoding...");
            let opcode = self.decode();
            println!("OK");

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
                    print!("Reading... ");
                    let value = self.input.next().await.expect("Read");
                    println!("OK");
                    self.store(value);
                }
                OpCode::Write => {
                    print!("Writing... ");
                    let value = self.load();
                    self.output.send(value).await.expect("Write");
                    println!("OK");
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
                _ => return Err(anyhow!("Mfuck")),
            };
        }
        Ok(())
    }
}

fn main() -> Result<()> {

    let f = File::open("07/input").unwrap();
    let file = BufReader::new(&f);
    let mut max = 0;
    let mut memory: Vec<isize> = file
        .lines()
        .into_iter()
        .filter_map(|x| x.ok())
        .flat_map(|s| {
            s.split(",")
                .map(|x| x.clone().parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect();

//    let part_a_mem = memory.clone();
//
//    let part_one = task::spawn(async move {
//        println!("Running part A");
//        for a in 0..5 {
//            for b in 0..5 {
//                if a == b {continue}
//                for c in 0..5 {
//                    if a == c  || b == c {continue}
//                    for d in 0..5 {
//                        if a == d  || b == d || c == d {continue}
//                        for e in 0..5 {
//                            if a == e  || b == e || c == e || d == e {continue}
//                            let phases = vec![a, b, c, d, e];
//                            let mut phase = phases.clone().into_iter();
//                            let (mut input, recv) = channel::<isize>(2);
//                            input.send(phase.next().unwrap()).await.expect("Phase Write");
//                            input.send(0).await.expect("Init Level Write");
//
//                            let mut recv: Option<Receiver<isize>> = Some(recv);
//
//                            let mut machines = vec![];
//                            for v in 0..5 {
//                                let (mut out, recv_next) = channel(2);
//                                if v < 4 {
//                                    out.send(phase.next().unwrap()).await.expect("Phase Write");
//                                }
//                                machines.push(IntCodeMachine::new(part_a_mem.clone(), recv.take().unwrap(), out));
//                                recv = Some(recv_next)
//                            }
//
//                            for m in machines.iter_mut() {
//                                m.run().await;
//                            }
//
//                            let output = recv.take().unwrap().next().await.unwrap();
//
//                            if output > max {
//                                max = output
//                            }
//                        }
//                    }
//                }
//            }
//        }
//        println!("max output A {}", max);
//    });
//    task::block_on(part_one);

    let part_two = task::spawn(async move {
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
                            let (mut input, recv) = channel::<isize>(2);
                            let phase_zero = phase.next().unwrap();

                            let mut recv: Option<Receiver<isize>> = Some(recv);

                            let mut machines = vec![];
                            for v in 0..5 {
                                let (mut out, recv_next) = channel(2);
                                if v < 4 {
                                    out.send(phase.next().unwrap()).await.expect("Phase Write");
                                }
                                machines.push(IntCodeMachine::new(memory.clone(), recv.take().unwrap(), out));
                                recv = Some(recv_next)
                            }

                            machines[0].input = recv.unwrap();
                            machines[4].output.send(phase_zero).await.unwrap();
                            machines[4].output.send(0).await.unwrap();

                            for m in machines.iter_mut() {
                                println!("Running machine");
                                m.run().await;
                            }

                            let output = machines[0].input.next().await.unwrap();
                            println!("phases {:?} output {} ", phases, output);
                            if output > max {
                                max = output
                            }
                        }
                    }
                }
            }
        }
        println!("max output B {}", max);
    });
    task::block_on(part_two);

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::IntCodeMachine;

    #[test]
    fn tests() {

    }
}
