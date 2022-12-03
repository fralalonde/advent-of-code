use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;

fn main() -> io::Result<()> {
    let f = File::open("day01/input")?;
    let file = BufReader::new(&f);
    let mut groups = vec![];
    let mut curr = vec![];

    for line in file.lines().into_iter()
        .filter_map(|x| x.ok()).into_iter()
    {
        if line.is_empty() {
            groups.push(curr);
            curr = vec![];
        } else {
            let p = line.parse::<u32>().unwrap();
            curr.push(p);
        }
    }
    groups.push(curr);

    let mut sums: Vec<_> = groups.into_iter().map(|v| v.into_iter().sum::<u32>()).collect();
    sums.sort();
    println!("part 1 {}", sums.iter().rev().next().unwrap());
    println!("part 2 {}",  sums.iter().rev().into_iter().take(3).sum::<u32>());

    Ok(())
}

