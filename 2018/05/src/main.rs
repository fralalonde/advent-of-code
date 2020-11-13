use std::fs::File;
use std::io::{BufReader, self};
use std::io::Read;

use std::i32;

const NONE: i32 = -1;

fn main() -> io::Result<()> {

    let f = File::open("input")?;
    let file = BufReader::new(&f);
    let mut orig: Vec<i32> = vec![];
    for byte in file.bytes().filter_map(|b| b.ok()) {
        orig.push(byte as i32);
    }

    let bytes = orig.clone();
    println!("burn 0 len {}", reduce(0, bytes));

    for i in 65..91 {
        let bytes = orig.clone();
        println!("burn {} 0 len {}", String::from_utf8(vec![i as u8]).unwrap(), reduce(i as i32, bytes));
    }

    Ok(())
}

fn reduce(burn: i32, mut bytes: Vec<i32>) -> usize {
    let mut culled;
    'repeat: loop {
        culled = 0;
        let mut prev_idx = 0;
        'main: for i in 0..bytes.len() {
            if bytes[i] == NONE {
                continue;
            }
            if bytes[i] == burn || bytes[i] == burn + 32 {
                bytes[i] = NONE;
                continue;
            }
            if prev_idx == NONE {
                // no prev, use curr
                prev_idx = i as i32;
                continue
            }
            if (bytes[prev_idx as usize] - bytes[i]).abs() == 32 {
                bytes[prev_idx as usize] = NONE;
                bytes[i] = NONE;
                culled += 2;
                while prev_idx > 0 {
                    prev_idx -= 1;
                    if bytes[prev_idx as usize] != NONE {
                        continue 'main;
                    }
                }
            } else {
                // undestroyed, use curr as prev before moving to next
                prev_idx = i as i32;
            }
        }
        if culled == 0 {
            break
        }
    }

    bytes.iter()
        .filter_map(|x| if *x == NONE { None } else { Some(*x as u8) }).count()
}
