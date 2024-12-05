use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() -> anyhow::Result<()> {
    let f = File::open(format!("{}/input", env!("CARGO_PKG_NAME")))?;
    let start = Instant::now();
    let pp = parse(f)?;
    println!("parsed [{}ns]", start.elapsed().as_nanos());

    let start = Instant::now();
    let p1 = part1(&pp)?;
    println!("part 1 {} [{}ns]", p1, start.elapsed().as_nanos());

    let start = Instant::now();
    let p2 = part2(&pp)?;
    println!("part 2 {} [{}ns]", p2, start.elapsed().as_nanos());

    Ok(())
}

pub enum Token {
    Do,
    Dont,
    Mul(isize, isize),
}

fn parse_mul(input: &str) -> Option<(Token, &str)> {
    if input.starts_with("mul(") {
        let rest = &input[4..];
        let mut parts = rest.splitn(2, ',');
        if let (Some(lhs), Some(rest)) = (parts.next(), parts.next()) {
            if let Some(idx) = rest.find(')') {
                let rhs = &rest[..idx];
                let remainder = &rest[idx + 1..];
                if let (Ok(lhs), Ok(rhs)) = (lhs.parse::<isize>(), rhs.parse::<isize>()) {
                    return Some((Token::Mul(lhs, rhs), remainder.trim_start()));
                }
            }
        }
    }
    None
}

fn parse_do(input: &str) -> Option<(Token, &str)> {
    if input.starts_with("do()") {
        return Some((Token::Do, &input[4..].trim_start()));
    }
    None
}

fn parse_dont(input: &str) -> Option<(Token, &str)> {
    if input.starts_with("don't()") {
        return Some((Token::Dont, &input[6..].trim_start()));
    }
    None
}

fn parse_tokens(mut input: &str, tokens: &mut Vec<Token>) {
    while !input.is_empty() {
        if let Some((token, rest)) = parse_mul(input)
            .or_else(|| parse_do(input))
            .or_else(|| parse_dont(input))
        {
            tokens.push(token);
            input = rest;
        } else {
            input = &input[1..];
        }
    }
}

pub fn parse(file: File) -> anyhow::Result<Vec<Token>> {
    let file = BufReader::new(&file);
    let mut list1 = vec![];

    for line in file.lines().into_iter()
        .filter_map(|x| x.ok())
        .into_iter()
    {
        parse_tokens(&line, &mut list1);
    }
    Ok(list1)
}


pub fn part1(list1: &[Token]) -> anyhow::Result<isize> {
    Ok(list1.iter().map(|t| match t {
        Token::Mul(x, y) => x * y,
        _ => 0
    }).sum())
}

pub fn part2(list1: &[Token]) -> anyhow::Result<isize> {
    let mut enabled = true;
    let mut sum = 0;
    for t in list1 {
        match t {
            Token::Do => {
                #[cfg(test)] println!("do");
                enabled = true
            }
            Token::Dont => {
                #[cfg(test)] println!("dont");
                enabled = false
            }
            Token::Mul(x, y) => if enabled {
                #[cfg(test)] println!("mult {} {}", x, y);
                sum += x * y
            } else {
                #[cfg(test)] println!("skip");
            },
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use crate::{*};

    #[test]
    fn p1() -> anyhow::Result<()> {
        let f = File::open("test")?;
        let pp = parse(f)?;

        let z = part1(&pp)?;
        assert_eq!(z, 161);

        Ok(())
    }

    #[test]
    fn p2() -> anyhow::Result<()> {
        let f = File::open("test2")?;
        let pp = parse(f)?;

        let z = part2(&pp)?;
        assert_eq!(z, 48);

        Ok(())
    }
}

