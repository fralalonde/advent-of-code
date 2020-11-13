mod a;
mod b;

use std::fs::File;
use std::io::{BufReader, BufRead, self};

fn main() -> io::Result<()> {
    let f = File::open("input")?;
    let file = BufReader::new(&f);
    let lines: Vec<i32> = file.lines().into_iter()
        .filter_map(|x| x.ok())
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    a::run(&lines);
    b::run(&lines);
    Ok(())
}
