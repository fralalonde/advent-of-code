use anyhow::*;
use std::fs::File;
use std::io::{BufReader, Read, BufRead};

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const LEN: usize = WIDTH * HEIGHT;

fn main() -> Result<()> {
    let f = File::open("08/input")?;
    let file = BufReader::new(&f);
    let bytes: Vec<char> = file.lines()
        .filter_map(|x| x.ok())
        .next().unwrap().chars()
        .collect();
    let mut least_zeroes = std::usize::MAX;
    let mut ones_times_twos = 0;
    let mut zeroes = 0;
    let mut ones = 0;
    let mut twos = 0;
    let mut bcount = 0;
    let mut pixels = Vec::with_capacity(150);
    for _ in 0..LEN {
        pixels.push('2');
    }
    for b in bytes {
        match b {
            '0' => zeroes = zeroes + 1,
            '1' => ones += 1,
            '2' => twos += 1,
            _ => {}
        }
        if pixels[bcount] == '2' { pixels[bcount] = b }
        bcount += 1;
        if bcount == LEN {
            if zeroes < least_zeroes {
                least_zeroes = zeroes;
                ones_times_twos = ones * twos
            }
            zeroes = 0;
            ones = 0;
            twos = 0;
            bcount = 0;
        }
    }

    println!("one * 2 {}", ones_times_twos);

    println!();
    for i in 1..LEN + 1 {
        match pixels[i - 1] {
            '1' => print!("\u{2588}\u{2588}"),
            _ => print!("  "),
        }
        if i % WIDTH == 0 {
            println!()
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {

    #[test]
    fn tests() {}
}
