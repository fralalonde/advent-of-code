mod a;
mod b;

use std::fs::File;
use std::io::{BufReader, BufRead, self};

fn main() -> io::Result<()> {
    let f = File::open("input")?;
    let file = BufReader::new(&f);
    let mut lines: Vec<Vec<u8>> = file.lines().into_iter()
        .filter_map(|x| x.ok())
        .map(|s| s.into_bytes())
        .collect();

    a::run(&mut lines.clone());
    b::run(&mut lines);
    Ok(())
}
