extern crate core;

mod parser;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::str::FromStr;
use std::time::Instant;
use anyhow::Result;

fn main() -> Result<()> {
    let f = File::open(format!("{}/input", env!("CARGO_PKG_NAME")))?;
    let pp = read(f)?;

    let start = Instant::now();

    println!("parsed [{}ns]", start.elapsed().as_nanos());

    let start = Instant::now();
    let p1 = part1(&pp.0, pp.1.clone())?;
    println!("part 1 {} [{}ns]", p1, start.elapsed().as_nanos());

    let start = Instant::now();
    let p2 = part2(&pp)?;
    println!("part 2 {} [{}ns]", p2, start.elapsed().as_nanos());

    Ok(())
}

#[derive(Clone, Debug)]
struct Grid {
    width: isize,
    height: isize,
    grid: HashSet<(isize, isize)>,
}

#[derive(Clone, Debug)]
struct Guard {
    pos: (isize, isize),
    dir: usize, // modulo 4 =  0N 1E 2S 3W
}

pub fn read<'i>(file: File) -> Result<(Grid, Guard)> {
    let file = BufReader::new(&file);
    let mut width = 0;
    let mut height = 0;
    let mut grid = HashSet::new();
    let mut guard = Guard { pos: (0, 0), dir: 0 };

    for (row, line) in file.lines().into_iter()
        .filter_map(|x| x.ok())
        .filter(|x| !x.is_empty())
        .into_iter().enumerate()
    {
        #[cfg(test)] println!("line {line}");
        width = line.len() as isize;
        height = (row + 1) as isize;
        for (col, ch) in line.as_str().as_bytes().iter().enumerate() {
            match *ch {
                b'#' => _ = grid.insert((col as isize, row as isize)),
                b'^' => guard = Guard { pos: (col as isize, row as isize), dir: 0 },
                b'>' => guard = Guard { pos: (col as isize, row as isize), dir: 1 },
                b'v' => guard = Guard { pos: (col as isize, row as isize), dir: 2 },
                b'<' => guard = Guard { pos: (col as isize, row as isize), dir: 3 },
                _ => {}
            }
        }
    }

    Ok((Grid { width, height, grid }, guard))
}

fn walk(grid: &Grid, mut guard: Guard) -> usize {
    let mut walked = HashSet::new();

    #[cfg(test)] println!("guard {guard:?} grid {grid:?}");
    loop {
        walked.insert(guard.pos);
        // one step
        loop {
            let next = match guard.dir % 4 {
                0 => (guard.pos.0, guard.pos.1 - 1),
                1 => (guard.pos.0 + 1, guard.pos.1),
                2 => (guard.pos.0, guard.pos.1 + 1),
                3 => (guard.pos.0 - 1, guard.pos.1),
                _ => panic!("wtf")
            };
            if (grid.grid.contains(&next)) {
                guard.dir += 1;
                continue;
            } else {
                guard.pos = next;
                break;
            }
        }

        if guard.pos.0 < 0 || guard.pos.0 >= grid.width || guard.pos.1 < 0 || guard.pos.1 >= grid.height {
            #[cfg(test)]{
                for i in 0..grid.height {
                    for j in 0..grid.width {
                        if grid.grid.contains(&(j, i)) {
                            print!("# ")
                        } else if walked.contains(&(j, i)) {
                            print!("X ")
                        } else {
                            print!(". ")
                        }
                    }
                    println!()
                }
            }
            // println!("walked {guard:?} grid {grid:?}");
            return walked.len();
        }
    }
}

pub fn part1(grid: &Grid, mut guard: Guard) -> Result<usize> {
    Ok(walk(grid, guard))
}

pub fn part2(data: &(Grid, Guard)) -> Result<usize> {
    let mut sum = 0;

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

        let z = part1(&pp.0, pp.1.clone())?;
        assert_eq!(z, 41);

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

