use std::io;
//use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use anyhow::*;
use std::collections::HashMap;

fn main() -> Result<()> {
    let f = File::open("10/input")?;
    let file = BufReader::new(&f);
    let wires: Vec<String> = file.lines().into_iter()
        .filter_map(|x| x.ok())
        .collect();

    let mut asteroids = HashMap::new();

    let mut y = 0;
    for line in file.lines() {
        for (x, b) in line?.as_bytes().iter().enumerate() {
            match *b as char {
                '#' => {asteroids.insert(x, y);},
                _ => {}
            }
        }
        y += 1;
    }

    println!("min steps");
    Ok(())
}

#[cfg(test)]
mod test {

    #[test]
    fn tests() {
    }
}
