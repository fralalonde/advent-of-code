use std::io;
//use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn ratio(mass: i32) -> i32 {
    mass / 3 - 2
}

fn req_fuel(mass: i32) -> i32 {
    if mass > 0 {
        mass + req_fuel(ratio(mass))
    } else {
        0
    }
}

fn main() -> io::Result<()> {
    let f = File::open("01/input")?;
    let file = BufReader::new(&f);
    println!("A {}", file.lines().into_iter()
        .filter_map(|x| x.ok())
        .filter_map(|s| s.parse::<i32>().ok())
        .map(|x| x / 3 - 2)
        .sum::<i32>());

    let f = File::open("01/input")?;
    let file = BufReader::new(&f);
    println!("B {}", file.lines().into_iter()
        .filter_map(|x| x.ok())
        .filter_map(|s| s.parse::<i32>().ok())
        .map(|x| x / 3 - 2)
        .map(req_fuel)
        .sum::<i32>());

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{req_fuel, ratio};

    #[test]
    fn wog() {
        assert_eq!(2, req_fuel(ratio(14)));
        assert_eq!(966, req_fuel(ratio(1969)))
    }
}
