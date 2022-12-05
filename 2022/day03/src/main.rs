use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;


fn pri(by: u8) -> u8 {
    if by > 96 {
        by - 96
    } else {
        by - 38
    }
}

fn dup(a: &[u8], b: &[u8]) -> u8 {
    for aa in a {
        if b.contains(aa) {
            return *aa;
        }
    }
    panic!("no dup")
}

fn badge(a: &[u8], b: &[u8], c: &[u8]) -> u8 {
    for aa in a {
        if b.contains(aa) && c.contains(aa) {
            return *aa;
        }
    }
    use std::str;
    panic!("no trip {:?} {:?} {:?}", str::from_utf8(a), str::from_utf8(b), str::from_utf8(c))
}

fn main() -> io::Result<()> {
    let f = File::open("day03/input")?;
    let file = BufReader::new(&f);

    let score = file.lines().into_iter()
        .filter_map(|x| x.map(|z| z.into_bytes()).ok()).into_iter()
        .map(|bytes| {
            let z = bytes.len() / 2;
            let a = &bytes[..z];
            let b = &bytes[z..];
            pri(dup(a, b)) as usize
        }).sum::<usize>();

    println!("score 1 {}", score);

    let f = File::open("day03/input")?;
    let file = BufReader::new(&f);

    let score = file.lines().into_iter()
        .filter_map(|x| x.map(|z| z.into_bytes()).ok()).into_iter()
        .fold((vec![], 0usize), |mut acc: (Vec<Vec<u8>>, usize), line| {
            acc.0.push(line);
            if acc.0.len() == 3 {
                acc.1 += pri(badge(acc.0[0].as_ref(), acc.0[1].as_ref(), acc.0[2].as_ref())) as usize;
                (vec![], acc.1)
            } else {
                acc
            }
        }).1;

    println!("score 2 {}", score);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{pri, badge};

    #[test]
    fn priaaa() {
        assert_eq!(pri("a".as_bytes()[0]), 1);
        assert_eq!(pri("A".as_bytes()[0]), 27);
        assert_eq!(pri("A".as_bytes()[0]), 27);
    }

    #[test]
    fn badger() {
        assert_eq!(18, pri(badge("vJrwpWtwJgWrhcsFMMfFFhFp".as_bytes(),
                                 "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".as_bytes(),
                                 "PmmdzqPrVvPwwTWBwg".as_bytes())));
        assert_eq!(52, pri(badge("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".as_bytes(),
                                 "ttgJtRGJQctTZtZT".as_bytes(),
                                 "CrZsJsPPZsGzwwsLwLmpwMDw".as_bytes())));
    }
}