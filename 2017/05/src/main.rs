extern crate failure;
//#[macro_use] extern crate failure_derive;

use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    run();
    run2();
}

fn run() -> Result<(), failure::Error> {
    let f = File::open("data")?;
    let mut file = BufReader::new(&f);
    let mut points: Vec<(i32, i32)> = file.lines().map(|line| (line.unwrap().parse::<i32>().unwrap(), 0 as i32)).collect();

    let mut steps = 0;
    let mut next: i32 = 0;

    while next >= 0 && next < points.len() as i32 {
        let mut op = points.get_mut(next as usize).expect("oh!");
        next += op.0 + op.1;
        op.1 += 1;
        steps += 1;
        if steps % 1024 == 0 {
            println!("{:?} next ", next);
        }
    }

    println!("{:?} exit in steps" , steps);
    Ok(())
}

fn run2() -> Result<(), failure::Error> {
    let f = File::open("data")?;
    let mut file = BufReader::new(&f);
    let mut points: Vec<(i32, i32)> = file.lines().map(|line| (line.unwrap().parse::<i32>().unwrap(), 0 as i32)).collect();

    let mut steps = 0;
    let mut next: i32 = 0;

    while next >= 0 && next < points.len() as i32 {
        let mut op = points.get_mut(next as usize).expect("oh!");
        let offset = op.0 + op.1;
        next += offset;
        if offset < 3 {
            op.1 += 1;
        } else {
            op.1 -= 1;
        }
        steps += 1;
    }

    println!("{:?} exit in steps" , steps);
    Ok(())
}