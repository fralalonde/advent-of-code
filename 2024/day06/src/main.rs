extern crate core;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
    let p2 = part2(&pp.0, pp.1.clone())?;
    println!("part 2 {} [{}ns]", p2, start.elapsed().as_nanos());

    Ok(())
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct P(i32, i32);

#[derive(Clone, Debug)]
pub struct Grid {
    width: i32,
    height: i32,
    grid: HashSet<P>,
}

#[derive(Clone, Debug)]
pub struct Guard {
    pos: P,
    dir: usize, // modulo 4 =  0N 1E 2S 3W
}

pub fn read<'i>(file: File) -> Result<(Grid, Guard)> {
    let file = BufReader::new(&file);
    let mut width = 0;
    let mut height = 0;
    let mut grid = HashSet::new();
    let mut guard = Guard { pos: P(0, 0), dir: 0 };

    for (row, line) in file.lines().into_iter()
        .filter_map(|x| x.ok())
        .filter(|x| !x.is_empty())
        .into_iter().enumerate()
    {
        #[cfg(test)] println!("line {line}");
        width = line.len() as i32;
        height = (row + 1) as i32;
        for (col, ch) in line.as_str().as_bytes().iter().enumerate() {
            match *ch {
                b'#' => _ = grid.insert(P(col as i32, row as i32)),
                b'^' => guard = Guard { pos: P(col as i32, row as i32), dir: 0 },
                b'>' => guard = Guard { pos: P(col as i32, row as i32), dir: 1 },
                b'v' => guard = Guard { pos: P(col as i32, row as i32), dir: 2 },
                b'<' => guard = Guard { pos: P(col as i32, row as i32), dir: 3 },
                _ => {}
            }
        }
    }

    Ok((Grid { width, height, grid }, guard))
}

fn delta(d1: P, d2: P) -> i32 {
    (d2.0 - d1.0).abs() + (d2.1 - d1.1).abs()
}

fn walk(grid: &Grid, mut guard: Guard) -> (usize, usize) {
    let mut legs: [P; 4] = [P(0, 0), P(0, 0), P(0, 0), P(0, 0)];
    // let mut head = 0;
    let mut loops = 0;

    let mut walked = HashSet::new();

    #[cfg(test)] println!("guard {guard:?} grid {grid:?}");
    loop {
        walked.insert(guard.pos);
        // one step
        loop {
            let next = match guard.dir % 4 {
                0 => P(guard.pos.0, guard.pos.1 - 1),
                1 => P(guard.pos.0 + 1, guard.pos.1),
                2 => P(guard.pos.0, guard.pos.1 + 1),
                3 => P(guard.pos.0 - 1, guard.pos.1),
                _ => panic!("wtf")
            };
            if grid.grid.contains(&next) {
                // turn right
                guard.dir += 1;

                // part2 - circ buffer
                let head = guard.dir;
                let h = head % 4;
                legs[h] = guard.pos;
                if head > 3 {
                    let t1 = (head - 3) % 4;
                    let t2 = (head - 2) % 4;
                    let t3 = (head - 1) % 4;
                    let d1 = delta(legs[t1], legs[t2]);
                    let d2 = delta(legs[h], legs[t3]);

                    #[cfg(test)]println!("t1 {:?} -> t2 {:?} dist {}", legs[t1], legs[t2], d1);
                    #[cfg(test)]println!("t3 {:?} -> gu {:?} dist {}", legs[t3], legs[h], d2);

                    if d2 >= d1 {
                        #[cfg(test)]println!("loop after {} at {:?} minus {}", head, guard.pos, (d2 - d1).abs());
                        loops += 1;
                    }
                }

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
                        if grid.grid.contains(&P(j, i)) {
                            print!("# ")
                        } else if walked.contains(&P(j, i)) {
                            print!("X ")
                        } else {
                            print!(". ")
                        }
                    }
                    println!()
                }
            }
            println!("loops {loops}");
            // println!("walked {guard:?} grid {grid:?}");
            return (walked.len(), loops);
        }
    }
}

pub fn part1(grid: &Grid, guard: Guard) -> Result<usize> {
    Ok(walk(grid, guard).0)
}

pub fn part2(grid: &Grid, guard: Guard) -> Result<usize> {
    Ok(walk(grid, guard).1)
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
        let z = part2(&pp.0, pp.1.clone())?;
        assert_eq!(z, 6);

        Ok(())
    }
}

