use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use regex::Regex;

fn main() -> anyhow::Result<()> {
    let f = File::open("day01/input")?;
    let pp = parse(f)?;
    println!("part 1 {}", part1(&pp)?);
    println!("part 2 {}", part2(&pp)?);
    Ok(())
}

pub fn parse(file: File) ->  anyhow::Result<(Vec<isize>, Vec<isize>)> {
    let file = BufReader::new(&file);
    let mut list1 = vec![];
    let mut list2 = vec![];
    let re = Regex::new(r"(\d+)\s+(\d+)")?;
    for line in file.lines().into_iter()
        .filter_map(|x| x.ok()).into_iter()
    {
        if let Some(caps) = re.captures(&line) {
            #[cfg(test)]
            println!("{} {}", &caps[1], &caps[2]);
            list1.push(isize::from_str(&caps[1])?);
            list2.push(isize::from_str(&caps[2])?);
        } else {
            panic!("no match {}", line)
        }
    }
    list1.sort();
    list2.sort();
    Ok((list1, list2))
}

pub fn part1((list1, list2): &(Vec<isize>, Vec<isize>)) -> anyhow::Result<isize>  {
    let mut sum: isize = 0;
    for i in 0..list1.len() {
        #[cfg(test)]
        println!("{} - {} = {}", list2[i], list1[i], (list2[i] - list1[i]).abs());
        sum += (list2[i] - list1[i]).abs();
    }
    Ok(sum)
}

pub fn part2((list1, list2): &(Vec<isize>, Vec<isize>)) -> anyhow::Result<isize>  {
    let mut sum: isize = 0;
    for n in list1 {
        let occ = list2.iter().filter(|n2| *n2 == n).count() as isize;
        let sim = *n * occ;
        #[cfg(test)]
        println!("{} * {} = {}", n, occ, sim);
        sum += sim;
    }
    Ok(sum)
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use crate::{parse, *};

    #[test]
    fn p1() -> anyhow::Result<()> {
        let f = File::open("test")?;
        println!("parse");
        let pp = parse(f)?;

        println!("part1");
        let z = part1(&pp)?;
        assert_eq!(z, 11);

        println!("part2");
        let zz = part2(&pp)?;
        assert_eq!(zz, 31);

        Ok(())
    }
}

