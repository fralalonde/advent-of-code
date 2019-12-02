use std::io;
//use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use anyhow::*;

fn main() -> Result<()> {
    let f = File::open("02/input")?;
    let file = BufReader::new(&f);
    let mut ops = file.lines().into_iter()
        .filter_map(|x| x.ok())
        .flat_map(|s| s.split(",")
            .map(|x| x.clone().parse::<usize>().unwrap())
            .collect::<Vec<usize>>());

    let mut positions = HashMap::new();
    positions.insert(1, 12);
    positions.insert(2, 2);

    loop {
        match ops.next().unwrap() {
            1 => {
                let a: usize = *positions.get(&ops.next().with_context(|| "A")?).unwrap_or(&12);
                let b: usize = *positions.get(&ops.next().with_context(|| "B")?).unwrap_or(&2);
                let c: usize = ops.next().unwrap();
                positions.insert(c, a + b);
            }
            2 => {
                let a: usize = *positions.get(&ops.next().with_context(|| "A")?).unwrap_or(&12);
                let b: usize = *positions.get(&ops.next().with_context(|| "B")?).unwrap_or(&2);
                let c: usize = ops.next().unwrap();
                positions.insert(c, a * b);
            }
            99 => break,
            _ => return Err(anyhow!("Mfuck")),
        }
    }

    println!("0: {}", positions.get(&0).unwrap());

    Ok(())
}

#[cfg(test)]
mod test {

    #[test]
    fn wog() {
    }
}
