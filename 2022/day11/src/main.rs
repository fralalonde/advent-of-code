use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

use crate::Op::{Addx, Noop};

use std::path::PathBuf;

enum Op {
    Noop,
    Addx(i32),
}

fn run(buf: impl BufRead, offset: i32, period: i32, len: usize) -> anyhow::Result<Vec<i32>> {
    let mut x: i32 = 1;


    let ops: Vec<_> = buf.lines().filter_map(|x| x.ok()).into_iter()
        .map(|line| {
            if line.eq("noop") {
                Noop
            } else if line.starts_with("addx ") {
                let a = i32::from_str(&line[5..]).unwrap();
                Addx(a)
            } else { panic!("fsdfsdf") }
        }).collect();
    loop {
        panic!("not enough ops")
    }
}

fn main() -> anyhow::Result<()> {
    let mut day_input = PathBuf::from(std::env::current_exe()?.file_name().unwrap());
    day_input.push("input");
    let f = File::open(day_input)?;
    let file = BufReader::new(&f);

    let sig = run(file, 20, 40, 6)?;
    println!("signal sum: {}", sig.iter().sum::<i32>() - 220);

    Ok(())
}

#[cfg(test)]
mod tests {
    // use std::borrow::Borrow;
    use std::io::BufReader;
    use crate::{run};

    static INPUT0: &str = r"";

    #[test]
    fn tang() -> anyhow::Result<()> {
        let x_trace = run(BufReader::new(INPUT0.as_bytes()), 0, 1, 6)?;
        println!("trace {:?}", x_trace);
        Ok(())
    }
}