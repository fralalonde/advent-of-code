use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use strum::*;
use strum_macros::*;
use std::collections::{ HashMap};
use std::sync::mpsc::{Receiver, sync_channel, SyncSender};
use futures_executor::LocalPool;
use futures_channel::mpsc::*;
use futures::stream::Stream;


#[repr(usize)]
#[derive(EnumIter, Debug, Clone, Copy)]
enum PMode {
    Indirect = 0,
    Immediate = 1,
    Relative = 2,
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
    RelativeBase = 09,

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

const PAGE_SIZE: usize = 4096;

struct IntCodeMachine {
    cir: usize,
    rel: isize,
    pmodes: usize,
    pages: HashMap<usize, Vec<isize>>,
    input: Receiver<isize>,
    _output: SyncSender<isize>,
}

impl IntCodeMachine {
    fn new(input: Receiver<isize>, _output: SyncSender<isize>) -> Self {
        IntCodeMachine {
            cir: 0,
            rel: 0,
            pmodes: 0,
            pages: HashMap::new(),
            input,
            _output
        }
    }
    
    fn init(&mut self, program: &[isize]) {
        self.cir = 0;
        self.rel = 0;
        self.pages = HashMap::new();
        for i in program {
            self.push(*i);
        }
        self.cir = 0;        
    }

    fn log(&self, msg: &str) {
//        println!("{}: {}", self.name, msg)
    }

    fn load(&mut self) -> isize {
        let param = *self.paged(self.cir);
        self.cir += 1;
        match self.pmode() {
            PMode::Immediate => param,
            PMode::Indirect => *self.paged(param as usize),
            PMode::Relative => *self.paged((self.rel + param) as usize),
        }
    }

    fn store(&mut self, value: isize) {
        let param = *self.paged(self.cir);
        self.cir += 1;
        match self.pmode() {
            PMode::Immediate => panic!("Parameters that an instruction writes to will never be in immediate mode."),
            PMode::Indirect => *self.paged(param as usize) = value,
            PMode::Relative => *self.paged((self.rel + param) as usize) = value,
        }
    }

    fn push(&mut self, value: isize) {
        *self.paged(self.cir) = value;
        self.cir += 1;
    }

    fn new_page() -> Vec<isize> {
        Vec::with_capacity(PAGE_SIZE)
    }

    fn paged(&mut self, addr: usize) -> &mut isize {
        let page = addr / PAGE_SIZE;
        let offset = addr - (page * PAGE_SIZE);
        unsafe { self.pages.entry(page).or_insert(Self::new_page()).get_unchecked_mut(offset) }
    }

    fn decode(&mut self) -> OpCode {
        let op = *self.paged(self.cir) as usize;
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

    pub async fn run(mut self) -> Result<Self> {
        loop {
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
                    let value = self.input.try_recv().expect("Short Read");
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
                OpCode::RelativeBase => {
                    let a = self.load();
                    self.rel += a;
                }
                OpCode::Halt => break,
            };
        }
        Ok(self)
    }
}

fn main() -> Result<()> {
    let f = File::open("09/input").unwrap();
    let file = BufReader::new(&f);
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

    let (in_sender, in_recv) = sync_channel(2);
    let (out_sender, _out_recv) = sync_channel(2);
    let mut machine = IntCodeMachine::new(in_recv, out_sender);


    let mut pool = LocalPool::new();
    machine.init(&program);
    in_sender.send(1)?;
    machine = pool.run_until(machine.run())?;

    machine.init(&program);
    in_sender.send(2)?;
    machine = pool.run_until(machine.run())?;


    println!();
    println!("page size: {} pages: {}", PAGE_SIZE, machine.pages.len());

    Ok(())
}