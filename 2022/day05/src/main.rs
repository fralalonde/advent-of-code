use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;
use std::str::FromStr;

use regex::Regex;

fn main() -> anyhow::Result<()> {
    let f = File::open("day05/input")?;
    let file = BufReader::new(&f);

    let mut stacks = vec![vec![]; 9];
    let mut moves: Vec<(usize, usize, usize)> = vec![];

    let mut lines = file.lines().filter_map(|l| l.ok()).into_iter();
    loop {
        let line = lines.next().unwrap();
        if line.starts_with(" ") { break; }
        let line = line.as_bytes();
        for i in 0..line.len() {
            if line[i] > 64 && line[i] < 91 {
                stacks[i / 4].push(line[i]);
            }
        }
    }

    for s in &mut stacks {
        s.reverse()
    }

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)")?;

    while let Some(line) = lines.next() {
        if line.is_empty() { continue; }
        if let Some(caps) = re.captures(&line) {
            moves.push((usize::from_str(&caps[1])?, usize::from_str(&caps[2])?, usize::from_str(&caps[3])?));
        } else {
            panic!("no match {}", line)
        }
    }
    // unmut
    let stacks = stacks;

    let mut st = stacks.clone();

    for m in &moves {
        for _ in 0..m.0 {
            let z = st[m.1 - 1].pop().unwrap();
            st[m.2 - 1].push(z);
        }
    }
    use std::str;
    for s in &st {
        print!("{}", str::from_utf8(&s[s.len() - 1..])?);
    }
    println!();

    let mut st = stacks.clone();

    for m in &moves {
        // can't have two mut slices at once :( 
        let y = st[m.1 - 1].len() - m.0;
        let z = Vec::from(&st[m.1 - 1][y..]);
        st[m.1 - 1].truncate(y);
        st[m.2 - 1].extend_from_slice(&z);
    }

    for s in &st {
        print!("{}", str::from_utf8(&s[s.len() - 1..])?);
    }
    println!();


    // println!("score A {:?}", full);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn ting() {
        assert_eq!(0, 0);
    }
}