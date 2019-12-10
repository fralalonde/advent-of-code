use std::io;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn what(ops: &[usize], i: usize) -> usize {
    let idx = ops[i];
    ops[idx]
}

fn whoo(ops: &mut Vec<usize>, i: usize, set: usize) {
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
                let a = what(&pos, i + 1);
                let b = what(&pos, i + 2);
                whoo(&mut pos, i + 3, a + b)
            }
            2 => {
                let a = what(&pos, i + 1);
                let b = what(&pos, i + 2);
                whoo(&mut pos, i + 3, a * b)
            }
            99 => break,
            _ => return Err(anyhow!("Mfuck")),
        };
        i += 4;
    }
    Ok(pos[0])
}

fn main() -> Result<()> {
    let f = File::open("02/input")?;
    let file = BufReader::new(&f);
    let mut og: Vec<usize> = file
        .lines()
        .into_iter()
        .filter_map(|x| x.ok())
        .flat_map(|s| {
            s.split(",")
                .map(|x| x.clone().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();

    // part A
    println!("A: {}", run(og.clone(), 12, 2)?);

    // part B
    'zzz: for a in 0..99 {
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
    fn wog() {}
}
