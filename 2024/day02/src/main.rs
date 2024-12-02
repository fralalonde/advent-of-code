use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use regex::Regex;

fn main() -> anyhow::Result<()> {
    let f = File::open(format!("{}/input", env!("CARGO_PKG_NAME")))?;
    let pp = parse(f)?;
    println!("part 1 {}", part1(&pp)?);
    println!("part 2 {}", part2(&pp)?);
    Ok(())
}

pub fn parse(file: File) -> anyhow::Result<Vec<Vec<isize>>> {
    let file = BufReader::new(&file);
    let mut list1 = vec![];


    let re = Regex::new(r"(\d+)").unwrap();
    for line in file.lines().into_iter()
        .filter_map(|x| x.ok())
        .into_iter()
    {
        let list2: Vec<isize> = re.captures_iter(&line)
            .map(|cap| isize::from_str(cap.get(1).unwrap().as_str()).unwrap())
            .collect();

        #[cfg(test)]
        println!("{:?}", &list2);
        list1.push(list2);
    }
    Ok(list1)
}

pub fn safe(list2: &Vec<isize>, dampen: bool) -> bool {
    let plain = increasing(list2) || decreasing(list2);
    if plain {
        return true
    }
    if dampen {
        for i in 0..list2.len() {
            let mut l2 = list2.clone();
            l2.remove(i);
            if increasing(&l2) || decreasing(&l2) {
                return true;
            }
        }
    }
    false
}

pub fn increasing(list2: &Vec<isize>) -> bool {
    let mut list2 = list2.clone();
    for i in 0..list2.len() - 1 {
        let d = list2[i + 1] - list2[i];
        if d > 3 || d < 1 {
            return false;
        }
    }
    true
}

pub fn decreasing(list2: &Vec<isize>) -> bool {
    let mut list2 = list2.clone();
    for i in 0..list2.len() - 1 {
        let d = list2[i] - list2[i + 1];
        if d > 3 || d < 1 {
            return false;
        }
    }
    true
}

pub fn part1(list1: &Vec<Vec<isize>>) -> anyhow::Result<usize> {
    Ok(list1.iter().filter(|l| safe(&l, false)).count())
}

pub fn part2(list1: &Vec<Vec<isize>>) -> anyhow::Result<usize> {
    Ok(list1.iter().enumerate().filter(|(n, l)| {
        #[cfg(test)] println!("line {}", n);
        safe(&l, true)
    }).count())
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use crate::{parse, *};

    #[test]
    fn p1() -> anyhow::Result<()> {
        let f = File::open("test")?;
        let pp = parse(f)?;

        let z = part1(&pp)?;
        assert_eq!(z, 2);

        Ok(())
    }

    #[test]
    fn p2() -> anyhow::Result<()> {
        let f = File::open("test")?;
        let pp = parse(f)?;

        let z = part2(&pp)?;
        assert_eq!(z, 4);

        Ok(())
    }
}

