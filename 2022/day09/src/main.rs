use std::cmp::max;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use regex::internal::Input;
use regex::Regex;
// use ascii::AsciiString;
// use ascii::AsAsciiStr;
// use lazy_static::lazy_static;

///     let ascii = read_ascii(file);
///     let _ = ascii[0].get_ascii(0);
// fn read_ascii(buf: impl BufRead) -> Vec<AsciiString> {
//     buf.lines().filter_map(|x| x.ok()).into_iter()
//         .map(|x| AsciiString::from_ascii(x).unwrap())
//         .collect()
// }

// fn read_strings(buf: impl BufRead) -> Vec<String> {
//     buf.lines().filter_map(|x| x.ok()).into_iter()
//         .collect()
// }

// fn read_bytes(buf: impl BufRead) -> Vec<Vec<u8>> {
//     buf.lines().filter_map(|x| x.ok()).into_iter()
//         .map(|x| x.into_bytes())
//         .collect()
// }
use linked_hash_set::LinkedHashSet;

fn follow(head: (i32, i32), mut tail: &mut (i32, i32)) {
    let gap = max(i32::abs(head.0 - tail.0), i32::abs(head.1 - tail.1));
    let g = if gap > 1 { 0 } else { 1 };
    if head.0 - tail.0 > g {
        tail.0 += 1;
    } else if head.0 - tail.0 < -1 * g {
        tail.0 -= 1
    }
    if head.1 - tail.1 > g {
        tail.1 += 1;
    } else if head.1 - tail.1 < -1 * g {
        tail.1 -= 1
    }
}

fn mv(head: &mut (i32, i32), rel: (i32, i32)) {
    head.0 += rel.0;
    head.1 += rel.1
}

fn animate(file: impl BufRead, rlen: usize) -> anyhow::Result<Vec<LinkedHashSet<(i32, i32)>>> {
    let re: Regex = Regex::new(r"(\w) (\d+)")?;
    let moves = file.lines().filter_map(|x| x.ok()).into_iter()
        .map(|s| {
            let caps = re.captures(&s).unwrap();
            (caps[1].to_owned(), i32::from_str(&caps[2]).unwrap())
        })
        .collect::<Vec<(String, i32)>>();

    let mut visited = vec![LinkedHashSet::new(); rlen];
    let mut rope = vec![(0, 0); rlen + 1];

    for i in 0..rlen  {
        visited[i].insert(rope[i]);
    }

    for m in &moves {
        let rel = match m.0.as_ref() {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => panic!("fff")
        };

        for i in 0..m.1 {
            mv(&mut rope[0], rel);


            for i in 0..rlen  {

                follow(rope[i], &mut rope[i + 1]);
                visited[i].insert(rope[i + 1]);
                if rope[i] != rope[i + 1] {
                    println!("A {} {}/{} :: H {:?}  T {:?}", m.0, i + 1, m.1, rope[i], rope[i + 1]);
                }
            }
        }
    }

    Ok(visited)
}

fn main() -> anyhow::Result<()> {
    let f = File::open("day09/input")?;
    let file = BufReader::new(&f);
    let visited = animate(file, 1)?;
    println!("short count: {}", visited.last().unwrap().len());

    let f = File::open("day09/input")?;
    let file = BufReader::new(&f);
    let visited = animate(file, 9)?;
    println!("long count: {}", visited.last().unwrap().len());

    Ok(())
}

#[cfg(test)]
mod tests {
    // use std::borrow::Borrow;
    use std::io::BufReader;
    use regex::internal::Input;
    use crate::animate;

    static INPUT1: &str = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn ting() -> anyhow::Result<()> {
        let set = animate(BufReader::new(INPUT1.as_bytes()), 1)?;
        println!("{set:?}");
        assert_eq!(13, set.last().unwrap().len());
        Ok(())
    }

    static INPUT2: &str = r"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn tang() -> anyhow::Result<()> {
        let set = animate(BufReader::new(INPUT2.as_bytes()), 9)?;
        println!("{set:?}");
        assert_eq!(36, set.last().unwrap().len());
        Ok(())
    }
}