#![feature(coroutines, coroutine_trait, stmt_expr_attributes)]

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use std::ops::{Coroutine, CoroutineState};
use std::ops::CoroutineState::Yielded;
use std::pin::Pin;

fn main() -> anyhow::Result<()> {
    let f = File::open(format!("{}/input", env!("CARGO_PKG_NAME")))?;
    let start = Instant::now();
    let pp = parse(f)?;
    println!("parsed [{}ns]", start.elapsed().as_nanos());

    let start = Instant::now();
    let p1 = part1(&pp)?;
    println!("part 1 {} [{}ns]", p1, start.elapsed().as_nanos());

    let start = Instant::now();
    let p2 = part2(&pp)?;
    println!("part 2 {} [{}ns]", p2, start.elapsed().as_nanos());

    Ok(())
}

pub struct Grid {
    pub data: Vec<Vec<u8>>,
}

impl Grid {
    pub fn rot45ccw(&self) -> Vec<Vec<u8>> {
        let mut lines = vec![];

        let height = self.data.len();
        let width = self.data[0].len();

        for start in 0..width {
            let mut line = vec![];
            let mut col = start;
            for row in 0..height {
                col -= 1;
                if col < 0 { break; }
                line.push(self.data[row][col]);
            }
            lines.push(line)
        }

        for start in 1..height {
            let mut line = vec![];
            let mut col = start;
            for row in 0..height {
                col -= 1;
                if col < 0 { break; }
                line.push(self.data[row][col]);
            }
            lines.push(line)
        }

        lines
    }
}

pub fn parse(file: File) -> anyhow::Result<Grid> {
    let file = BufReader::new(&file);
    Ok(Grid {
        data: file.lines().into_iter()
            .filter_map(|x| x.ok())
            .map(|line| line.as_bytes().to_vec())
            .collect()
    })
}

const XMAS: &[u8] = b"XMAS";
const SAMX: &[u8] = b"SAMX";

fn count(line: &[u8], pattern: &[u8]) -> usize {
    line.windows(pattern.len())
        .filter(|window| window.starts_with(pattern))
        .count()
}

fn count2(line: &[u8], pattern: &[u8]) -> usize {
    line.windows(pattern.len())
        .filter(|window| window.starts_with(pattern))
        .count()
}

pub fn part1(grid: &Grid) -> anyhow::Result<usize> {
    let mut sum = 0;
    sum += grid.data.iter().map(|line| count(line, XMAS) + count(line, SAMX)).sum::<usize>();
    #[cfg(test)] println!("0 {}", sum);
    sum += grid.rot90().map(|line| count(&line, XMAS) + count(&line, SAMX)).sum::<usize>();
    #[cfg(test)] println!("90 {}", sum);
    let mut v = grid.rot45ccw();
    while let Yielded(line) = Pin::new(&mut v).resume(()) {
        sum += count2(line, XMAS) + count2(line, SAMX)
    }
    #[cfg(test)] println!("-45 {}", sum);
    sum += grid.rot45cw().map(|line| count(&line, XMAS) + count(&line, SAMX)).sum::<usize>();
    #[cfg(test)] println!("45 {}", sum);
    Ok(sum)
}

pub fn part2(grid: &Grid) -> anyhow::Result<isize> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use crate::{*};

    #[test]
    fn p1() -> anyhow::Result<()> {
        let f = File::open("test")?;
        let pp = parse(f)?;

        let z = part1(&pp)?;
        assert_eq!(z, 18);

        Ok(())
    }

    #[test]
    fn p2() -> anyhow::Result<()> {
        let f = File::open("test2")?;
        let pp = parse(f)?;

        let z = part2(&pp)?;
        assert_eq!(z, 48);

        Ok(())
    }
}

