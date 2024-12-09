extern crate core;

use std::fs::File;
use std::io::{BufRead, BufReader};

use std::str::FromStr;
use std::time::Instant;
use anyhow::Result;
use regex::Regex;
use strum_macros::FromRepr;

fn main() -> Result<()> {
    let f = File::open(format!("{}/input", env!("CARGO_PKG_NAME")))?;
    let pp = read(f)?;

    let start = Instant::now();

    println!("parsed [{}ns]", start.elapsed().as_nanos());

    let start = Instant::now();
    let p1 = part1(&pp)?;
    println!("part 1 {} [{}ns]", p1, start.elapsed().as_nanos());

    let start = Instant::now();
    let p2 = part2(&pp)?;
    println!("part 2 {} [{}ns]", p2, start.elapsed().as_nanos());

    Ok(())
}

pub fn read<'i>(file: File) -> Result<Vec<Vec<u64>>> {
    let file = BufReader::new(&file);
    let re = Regex::new(r"(\d+)")?;

    let list1 = file.lines().into_iter()
        .filter_map(|x| x.ok())
        .filter(|x| !x.is_empty())
        .into_iter().map(|line| re.captures_iter(&line)
        .map(|cap| {
            let z = cap.get(1).unwrap().as_str();
            u64::from_str(z).unwrap()
        })
        .collect::<Vec<u64>>())
        .collect();
    Ok(list1)
}

#[repr(u64)]
#[derive(FromRepr, Debug, PartialEq)]
pub enum Oper {
    Add,
    Mul,
    Con,
}

#[derive(Debug)]
struct BaseIter<const T: u64> {
    value: u64,
    op_count: u32,
}

impl<const T: u64> Iterator for BaseIter<T> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.op_count == 0 {
            return None;
        }
        self.op_count -= 1;
        let modu = self.value % T;
        self.value /= T;
        Some(modu)
    }
}

fn search<const T: u64>(target: u64, values: Vec<u64>) -> bool {
    let op_count = (values.len() - 1) as u32;
    let combinations = T.pow(op_count);
    #[cfg(test)] println!("target: {target} op_count {op_count} combinations {combinations}");
    for round in 0..combinations {
        let ops = BaseIter::<T> { value: round, op_count }.map(|b| Oper::from_repr(b).unwrap());
        let r2 = eval(&values, ops);
        #[cfg(test)]
        let vv: Vec<_> = BaseIter::<T> { value: round, op_count }.map(|b| Oper::from_repr(b).unwrap()).collect();
        #[cfg(test)] println!("{vv:?}");
        if r2 == target {
            #[cfg(test)] println!("found {target}");
            return true;
        }
    }
    false
}

fn eval(x: &[u64], ops: impl Iterator<Item=Oper>) -> u64 {
    let mut acc = x[0];
    for (i, op) in ops.enumerate() {
        let b = x[i + 1];
        match op {
            Oper::Add => acc = acc + b,
            Oper::Mul => acc = acc * b,
            Oper::Con => acc = u64::from_str(&format!("{acc}{b}")).unwrap(),
        }
    }
    acc
}

pub fn part1(data: &[Vec<u64>]) -> Result<u64> {
    let mut sum = 0;
    for eq in data {
        let mut parts = eq.into_iter();
        let rez = *parts.next().unwrap();
        let comps = parts.cloned().collect();
        if search::<2>(rez, comps) {
            sum += rez;
        }
    }
    Ok(sum)
}

pub fn part2(data: &[Vec<u64>]) -> Result<u64> {
    let mut sum = 0;
    for eq in data {
        let mut parts = eq.into_iter();
        let rez = *parts.next().unwrap();
        let comps = parts.cloned().collect();
        if search::<3>(rez, comps) {
            sum += rez;
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use crate::{*};

    #[test]
    fn p1() -> Result<()> {
        let f = File::open("test")?;
        let pp = read(f)?;

        let z = part1(&pp)?;
        assert_eq!(z, 3749);

        Ok(())
    }

    #[test]
    fn p2() -> Result<()> {
        let f = File::open("test")?;
        let pp = read(f)?;
        let z = part2(&pp)?;
        assert_eq!(z, 11387);

        Ok(())
    }
}

