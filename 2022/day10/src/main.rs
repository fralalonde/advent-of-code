use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

use crate::Op::{Addx, Noop};

enum Op {
    Noop(usize),
    Addx(usize, i32),
}

fn run(buf: impl BufRead, offset: i32, period: i32, len: usize) -> anyhow::Result<Vec<i32>> {
    let mut x: i32 = 1;
    let mut scan = vec![];


    let ops: Vec<_> = buf.lines().filter_map(|x| x.ok()).into_iter()
        .enumerate()
        .map(|(num, line)| {
            if line.eq("noop") {
                Noop(num)
            } else if line.starts_with("addx ") {
                let a = i32::from_str(&line[5..]).unwrap();
                Addx(num, a)
            } else { panic!("fsdfsdf") }
        }).collect();
    loop {
        let mut cyc = 0i32;

        for op in &ops {
            match op {
                Noop(num) => {
                    cyc += 1;
                    if (cyc - offset) % period == 0 {
                        println!("A({num}) cyc {cyc} x {x}");
                        scan.push(cyc as i32 * x);
                        if scan.len() == len {
                            return Ok(scan);
                        }
                    }
                }
                Addx(num, a) => {
                    cyc += 1;
                    if (cyc - offset) % period == 0 {
                        println!("B({num}) cyc {cyc} x {x}");
                        scan.push(cyc as i32 * x);
                        if scan.len() == len {
                            return Ok(scan);
                        }
                    }
                    cyc += 1;
                    if (cyc - offset) % period == 0 {
                        println!("C({num}) cyc {cyc} x {x}");
                        scan.push(cyc as i32 * x);
                        if scan.len() == len {
                            return Ok(scan);
                        }
                    }
                    x += a;
                }
            }
        }
        panic!("not enough ops {cyc}")
    }
}

fn main() -> anyhow::Result<()> {
    let mut day_input = PathBuf::from(std::env::current_exe()?.file_name().unwrap());
    day_input.push("input");
    let f = File::open(day_input)?;
    let file = BufReader::new(&f);

    let sig = run(file, 20, 40, 6)?;
    println!("signal sum: {}", sig.iter().sum::<i32>());

    Ok(())
}

#[cfg(test)]
mod tests {
    // use std::borrow::Borrow;
    use std::io::BufReader;
    use crate::{run};

    static INPUT0: &str = r"noop
addx 3
addx -5";

    #[test]
    fn tang() -> anyhow::Result<()> {
        let x_trace = run(BufReader::new(INPUT0.as_bytes()), 0, 1, 5)?;
        println!("trace {:?}", x_trace);
        Ok(())
    }

    static INPUT1: &str = r"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn ting() -> anyhow::Result<()> {
        let set = run(BufReader::new(INPUT1.as_bytes()), 20, 40, 6)?;
        println!("{set:?}");
        assert_eq!(13140, set.iter().sum::<i32>());
        Ok(())
    }
}