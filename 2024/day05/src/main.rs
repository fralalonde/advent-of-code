mod parser;

use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::time::Instant;
use anyhow::Result;
use crate::parser::FileData;

fn main() -> Result<()> {
    let mut f = File::open(format!("{}/input", env!("CARGO_PKG_NAME")))?;

    let mut input = String::new();
    f.read_to_string(&mut input)?;
    let start = Instant::now();
    if let Ok(mut pp) = parser::parse_file(&input) {
        println!("parsed [{}ns]", start.elapsed().as_nanos());

        let start = Instant::now();
        let p1 = part1(&pp.1)?;
        println!("part 1 {} [{}ns]", p1, start.elapsed().as_nanos());

        let start = Instant::now();
        let p2 = part2(&mut pp.1)?;
        println!("part 2 {} [{}ns]", p2, start.elapsed().as_nanos());
    }
    Ok(())
}

pub fn read<'i>(file: File) -> Result<String> {
    let mut file = BufReader::new(&file);
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(String::from_utf8(buffer)?)
}

fn is_valid(pages: &[u32], deps: &[(u32, u32)]) -> bool {
    let mut printed = vec![];
    for p in pages {
        if deps.iter().filter(|(d,pp)| pp == p && pages.contains(d) && !printed.contains(d)).next().is_some() {
            #[cfg(test)] println!("page {p}");
            return false;
        }
        printed.push(*p);
    }
    true
}

pub fn part1(data: &FileData) -> anyhow::Result<u32> {
    let sum = data.seqs.iter()
        .filter_map(|l|
            is_valid(&l, &data.rules)
                .then(|| l[l.len()/2]))
        .sum();

    Ok(sum)
}

pub fn part2(data: &mut FileData) -> anyhow::Result<u32> {
    let mut sum = 0;
    let rules = data.rules.clone();
    for seq in data.seqs.iter_mut() {
        let mut swaps = 0;
        #[cfg(test)] println!("seq {seq:?}");
        loop {
            'bzz:
            for i in 0..seq.len() {
                let p = seq[i];
                for d in &rules {
                    if d.1 == p && seq.contains(&d.0) {
                        let pos = seq.iter().position(|z| *z == d.0).unwrap();
                        if pos > i {
                            seq[i] = d.0;
                            seq[pos] = p;
                            #[cfg(test)] println!("swap {} < > {}", seq[i], seq[pos]);
                            swaps += 1;
                            continue 'bzz;
                        }
                    }
                }
            }
            if is_valid(seq, &rules) {break}
        }

        if swaps > 0 {

            let mid = seq[seq.len()/2];
            #[cfg(test)] println!("seq {seq:?} mid {mid}");
            assert!(is_valid(seq, &rules));
            sum += mid
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use crate::{*};

    #[test]
    fn p1() -> Result<()> {
        let f = File::open("test")?;
        let s = read(f)?;
        if let Ok(pp) = parser::parse_file(&s) {
            let z = part1(&pp.1)?;
            assert_eq!(z, 143);
        }
        Ok(())
    }

    #[test]
    fn p2() -> Result<()> {
        let f = File::open("test")?;
        let s = read(f)?;
        if let Ok(mut pp) = parser::parse_file(&s) {
            let z = part2(&mut pp.1)?;
            assert_eq!(z, 123);
        }
        Ok(())
    }
}

