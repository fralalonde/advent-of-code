use std::io;
//use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use anyhow::*;

fn wire(ops: &[usize], i: usize) -> usize {
    let idx = ops[i];
    ops[idx]
}

fn whoo(ops: &mut Vec<usize>, i: usize, set: usize){
    let idx = ops[i];
    ops[idx] = set
}

fn run(mut pos: Vec<usize>, a: usize, b: usize) -> Result<usize> {
    let mut i = 0;

    pos[1] = a;
    pos[2] = b;
    loop {
        let op = pos[i];
        match op {
            1 => {
                let a =  what(&pos, i + 1);
                let b =  what(&pos, i + 2);
                whoo(&mut pos, i+ 3,  a + b)
            }
            2 => {
                let a =  what(&pos, i + 1);
                let b =  what(&pos, i + 2);
                whoo(&mut pos, i+ 3,  a * b)
            }
            99 => break,
            _ => return Err(anyhow!("Mfuck")),
        };
        i += 4;
    }
    Ok(pos[0])
}

enum Direction {
    Up(len),
    Right(len),
    Down(len),
    Left(len),
}

impl Direction {
    fn parallel(&self, other: &Self) -> bool {
        (self.horizontal() && other.horizontal()) || (!self.horizontal() && !other.horizontal())
    }

    fn horizontal(&self) -> bool {
        self == Direction::Right || self == Direction::Left
    }
}

#[derive(Clone)]
struct Point {
    x: isize,
    y: isize,
}

struct Line {
    origin: Point,
    direct: Direction,
}

impl Line {
    fn intersect(&self, other: &Line) -> Option<Point> {
        if self.parallel(other) {return None}
        self.in_range(other) && other.in_range(self)
    }

    fn in_range(&self, other: &Line) -> Option<Point> {
        match self.direct {
            Direction::Up(z) => other.origin.x <= self.origin.x && other.origin.x >= (self.origin.x - z)
            Direction::Right(z) => other.origin.x <= self.origin.x && other.origin.x >= (self.origin.x - z)
            Direction::Down(z) => other.origin.x <= self.origin.x && other.origin.x >= (self.origin.x - z)
            Direction::Left(z) => other.origin.x <= self.origin.x && other.origin.x >= (self.origin.x - z)
        }
    }

    fn terminal(&self) -> Point {
        match self.direct {
            Direction::Up(len) =>  Point{x: self.x, y : self.y + len},
            Direction::Right(len) =>  Point{x: self.x + len, y : self.y},
            Direction::Down(len) =>  Point{x: self.x, y : self.y - len},
            Direction::Left(len) =>  Point{x: self.x - len, y : self.y},
        }
    }
}

fn main() -> Result<()> {
    let f = File::open("02/input")?;
    let file = BufReader::new(&f);
    let mut wires: Vec<String> = file.lines().into_iter()
        .filter_map(|x| x.ok())
        .collect();

    let mut lines: Vec<Vec<Line>> = vec![];
    let mut orig: Point = Point{x:0,y:0};
    for wire in wires {
        for lines in wire.split(",") {

        }
    }


    wires.flat_map(|s| s.split(",")
            .map(|x| x.clone().parse::<usize>().unwrap())
            .collect::<Vec<usize>>())
        .collect();

    // part A
    println!("A: {}", run(og.clone(), 12, 2)?);

    // part B
    'zzz:
    for a in 0..99 {
        for b in 0..99 {
            if run(og.clone(), a, b)? == 19690720 {
                println!("B {}", 100 * a + b);
                break 'zzz;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {

    #[test]
    fn wog() {
    }
}
