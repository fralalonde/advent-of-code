use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use std::pin::Pin;
use regex::bytes::Regex;

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

type Page = usize;

pub struct Rule(Page, Page);

pub fn parse(file: File) -> anyhow::Result<(Vec<Rule>, Vec<Page>)> {s
    let re = Regex::new(r"(\d+)|(\d+)")?;
    let file = BufReader::new(&file);
    let mut lines  = file.lines().into_iter();
    for line in &mut lines
        .filter_map(|x| x.ok())
        .map(|line| line.as_bytes().to_vec()) {
        if line.is_empty() {break}
    }
    for line in lines
        .filter_map(|x| x.ok())
        .map(|line| line.as_bytes().to_vec()) {
        if line.is_empty() {break}
    }

    Ok(

            .collect()
    )
}

pub fn part1(grid: &Grid) -> anyhow::Result<usize> {
    let mut sum = 0;

    Ok(sum)
}

pub fn part2(grid: &Grid) -> anyhow::Result<isize> {
    let mut sum = 0;

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
        assert_eq!(z, 143);

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

