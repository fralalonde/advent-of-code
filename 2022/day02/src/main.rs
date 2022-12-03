extern crate core;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;
use crate::LWD::{Draw, Lose, Win};
use crate::RPS::{Paper, Rock, Scissors};

#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl From<&str> for RPS {
    fn from(zug: &str) -> Self {
        match zug {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => unreachable!("fuuuuuuuu")
        }
    }
}

fn play(opp: RPS, out: LWD) -> RPS {
    match out {
        Draw => opp,
        Win => match opp {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
        Lose => match opp {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }
}


#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum LWD {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl From<&str> for LWD {
    fn from(zug: &str) -> Self {
        match zug {
            "X" => Lose,
            "Y" => Draw,
            "Z" => Win,
            _ => unreachable!("fuuuuuuuu")
        }
    }
}

impl From<(RPS, RPS)> for LWD {
    fn from((opp, play): (RPS, RPS)) -> Self {
        if opp == play {
            Draw
        } else if opp == Rock && play == Scissors {
            Lose
        } else if opp == Scissors && play == Rock {
            Win
        } else if play as u32 > opp as u32 {
            Win
        } else { Lose }
    }
}

fn points(lwd: LWD, play: RPS) -> u32 {
    lwd as u32 + play as u32
}

fn main() -> io::Result<()> {
    let f = File::open("day02/input")?;
    let file = BufReader::new(&f);

    let score = file.lines().into_iter()
        .filter_map(|x| x.ok()).into_iter()
        .map(|line| {
            let parts: Vec<_> = line.split(" ").collect();
            let opp = RPS::from(parts[0]);
            let play = RPS::from(parts[1]);
            let lwd = LWD::from((opp, play));
            let p = points(lwd, play);
            p
        }).sum::<u32>();

    println!("score 1 {}", score);

    let f = File::open("day02/input")?;
    let file = BufReader::new(&f);
    let score = file.lines().into_iter()
        .filter_map(|x| x.ok()).into_iter()
        .map(|line| {
            let parts: Vec<_> = line.split(" ").collect();
            let opp = RPS::from(parts[0]);
            let lwd = LWD::from(parts[1]);
            let play = play(opp, lwd);
            let p = points(lwd, play);
            p
        }).sum::<u32>();

    println!("score 2 {}", score);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{LWD, points, RPS};

    #[test]
    fn read() {
        assert_eq!(RPS::Rock, RPS::from("A"));
        assert_eq!(RPS::Rock, RPS::from("X"));
    }

    #[test]
    fn resu() {
        assert_eq!(LWD::Lose, LWD::from((RPS::Rock, RPS::Scissors)));
        assert_eq!(LWD::Win, LWD::from((RPS::Scissors, RPS::Rock)));
        assert_eq!(LWD::Draw, LWD::from((RPS::Rock, RPS::Rock)));
    }
}