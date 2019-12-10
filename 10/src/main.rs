use std::fs::File;
use std::io::{BufReader, BufRead};
use anyhow::*;
use std::collections::{ HashSet};
use linked_hash_set::LinkedHashSet;

fn main() -> Result<()> {
    let f = File::open("10/input")?;
    let file = BufReader::new(&f);

    let mut asteroids/*: HashSet<(isize, isize)>*/ = LinkedHashSet::new();

    let mut width = 0;
    let mut height = 0;
    for line in file.lines() {
        let line = line?.as_bytes().to_vec();
        width = line.len() as isize;
        for (x, b) in line.iter().enumerate() {
            match *b as char {
                '#' => {
                    let x = x as isize;
                    asteroids.insert((x as isize, height));
                },
                _ => {}
            }
        }
        height += 1;
    }

    let spiral = spiral_out((width * width) as usize);
    println!("spiral {:?}", spiral);

    let mut max_count = 0;
    for (x, y) in &asteroids {
        println!();
        println!("asteroid at ({},{})", x, y);
        let mut count = 0;
        let mut neigh = asteroids.clone();
        neigh.remove(&(*x, *y));
        for (dx, dy) in &spiral {
            let mut occulted = false;
            for i in 1..width {
                if neigh.remove(&(x + (dx * i), y + (dy * i))) {
                    if !occulted {
                        count += 1;
                        occulted = true;
                        println!("seeing ({},{})", x + (dx * i), y + (dy * i));
                    } else {
                        println!("not seeing ({},{})", x + (dx * i), y + (dy * i));
                    }
                }
            }
        }
        println!("remain {:?}", neigh);
        if count > max_count {
            max_count = count;
            println!("found max seen {} ({},{})", max_count, x, y);
        }
    }
    println!();
    println!("found max seen {}", max_count);
    println!("total asteroids {}", asteroids.len());
    Ok(())
}

fn spiral_out(length: usize) -> Vec<(isize, isize)> {
    let mut seg_len = 2;
    let mut len = 0;
    let mut x = -1;
    let mut y = 1;
    let mut points = vec![];
    'out: loop {
        for dir in 0..4 {
            for _ in 0..seg_len {
                match dir {
                    0 => x += 1,
                    1 => y -= 1,
                    2 => x -= 1,
                    3 => y += 1,
                    _ => panic!("crap")
                }
                points.push((x, y));
                len += 1;
                if len > length {break 'out}
            }
        }
        y += 1;
        x -= 1;
        seg_len += 2;
    }
    points
}

#[cfg(test)]
mod test {

    #[test]
    fn tests() {
    }
}
