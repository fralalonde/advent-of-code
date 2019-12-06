use std::io;
//use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use anyhow::*;

fn main() -> Result<()> {
    let f = File::open("07/input")?;
    let file = BufReader::new(&f);
    let wires: Vec<String> = file.lines().into_iter()
        .filter_map(|x| x.ok())
        .collect();

    println!("min steps");

    Ok(())
}

#[cfg(test)]
mod test {

    #[test]
    fn tests() {
    }
}
