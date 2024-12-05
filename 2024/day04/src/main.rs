use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

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

pub struct Pix (u8, usize, usize);

impl Grid {
    fn width(&self) -> usize {
        self.data[0].len()
    }

    fn height(&self) -> usize {
        self.data.len()
    }

    fn down_left(&self, mut col: usize, mut row: usize) -> Vec<Pix> {
        let mut line = vec![];
        while row < self.height() {
            line.push(Pix(self.data[row][col], row, col));
            row += 1;
            if col == 0 { break; }
            col -= 1;
        }
        line
    }

    fn down_right(&self, mut col: usize, mut row: usize) -> Vec<Pix> {
        let mut line = vec![];
        while row < self.height() && col < self.width() {
            line.push(Pix(self.data[row][col], row, col));
            row += 1;
            col += 1;
        }
        line
    }

    fn down(&self, col: usize) -> Vec<Pix> {
        let mut line = vec![];
        for row in 0..self.height() {
            line.push(Pix(self.data[row][col], row, col));
        }
        line
    }

    pub fn rot0(&self) -> Vec<Vec<Pix>> {
        let mut lines = vec![];
        for (row, line) in self.data.iter().enumerate() {
            lines.push(line.iter().enumerate().map(|(col, ch)| Pix(*ch, row, col)).collect())
        }
        lines
    }

    pub fn rot45ccw(&self) -> Vec<Vec<Pix>> {
        let mut lines = vec![];
        for col in 0..self.width() {
            lines.push(self.down_left(col, 0))
        }
        let edge = self.width() - 1;
        for row in 1..self.height() {
            lines.push(self.down_left(edge, row))
        }
        lines
    }

    pub fn rot45cw(&self) -> Vec<Vec<Pix>> {
        let mut lines = vec![];
        for col in (0..self.width()).rev() {
            lines.push(self.down_right(col, 0))
        }
        for row in 1..self.height() {
            lines.push(self.down_right(0, row))
        }
        lines
    }

    pub fn rot90(&self) -> Vec<Vec<Pix>> {
        let mut lines = Vec::with_capacity(self.width());
        for col in 0..self.width() {
            lines.push(self.down(col))
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

fn count(line: &[Pix], pattern: &[u8]) -> usize {
    line.windows(pattern.len())
        .map(|w| w.iter().map(|p| p.0).collect::<Vec<u8>>())
        .filter(|window| window.eq(pattern))
        .count()
}

const SAM: &[u8] = b"SAM";
const MAS: &[u8] = b"MAS";

fn center(line: &[Pix], pattern: &[u8]) -> Vec<(usize, usize)> {
    let mut centers = vec![];
    let off = pattern.len() / 2;
    for  i in line.windows(pattern.len())
        .map(|w| w.iter().map(|p| p.0).collect::<Vec<u8>>())
        .enumerate()
        .filter(|window| window.1.eq(pattern))
        .map(|w| w.0) {
        centers.push((line[i + off].1, line[i + off].2))
    }
    centers
}

pub fn part1(grid: &Grid) -> anyhow::Result<usize> {
    let mut sum = 0;
    sum += grid.rot0().iter().map(|line| count(&line, XMAS) + count(&line, SAMX)).sum::<usize>();
    #[cfg(test)] println!("0 {}", sum);
    sum += grid.rot90().iter().map(|line| count(&line, XMAS) + count(&line, SAMX)).sum::<usize>();
    #[cfg(test)] println!("90 {}", sum);
    sum += grid.rot45ccw().iter().map(|line| count(&line, XMAS) + count(&line, SAMX)).sum::<usize>();
    #[cfg(test)] println!("-45 {}", sum);
    sum += grid.rot45cw().iter().map(|line| count(&line, XMAS) + count(&line, SAMX)).sum::<usize>();
    #[cfg(test)] println!("45 {}", sum);
    Ok(sum)
}

pub fn part2(grid: &Grid) -> anyhow::Result<usize> {
    let mut centers1 = HashSet::new();

    for line  in &grid.rot45ccw() {
        centers1.extend(center(line, MAS));
        centers1.extend(center(line, SAM))
    }
    let mut centers2 = HashSet::new();
    for line  in &grid.rot45cw() {
        centers2.extend(center(line, MAS));
        centers2.extend(center(line, SAM))
    }

    Ok(centers2.intersection(&centers1).count())
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
        assert_eq!(z, 9);
        Ok(())
    }
}

