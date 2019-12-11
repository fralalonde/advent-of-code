use std::fs::File;
use std::io::{BufReader, BufRead};
use anyhow::*;
use linked_hash_set::LinkedHashSet;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::cmp::Ordering::{Greater, Less};
use std::f32::consts::PI;

fn main() -> Result<()> {
    let f = File::open("10/input")?;
    let file = BufReader::new(&f);

    let mut asteroids = LinkedHashSet::new();

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

    let spiral = spiral_out(width as usize);

    println!("Part A: find the fuckers");
    let mut base = (0, 0);
    let mut max_count = 0;
    for (x, y) in &asteroids {
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
                    }
                }
            }
        }
        if count > max_count {
            max_count = count;
            base = (*x, *y);
            println!("found max {} ({:?})", max_count, base);
        }
    }

    println!();
    println!("Part B:  shoot the fuckers from {:?}", base);
    let mut targets = HashMap::new();
    let mut angles = HashMap::new();
    for (x, y) in &asteroids {
        // don't shoot self
        if (x,y) == (&base.0, &base.1) { continue }
        let angle = ((y - base.1) as f32).atan2((x - base.0) as f32);
        let angle_name = format!("{}", angle); // rust Y U NO eq f32
        targets.entry(angle_name.clone()).or_insert(vec![]).push((x,y));
        angles.entry(angle_name).or_insert(angle);
    }

    let mut ang2: Vec<f32> = angles.values().map(|g| *g).collect();
    ang2.sort_by(|a, b| if *a > *b { Greater} else { Less });
    let mut started = false;
    let mut count = 0;
    'kill: loop {
        for a in &ang2 {
            if !started {
                if *a >= -PI/2.0 { started = true} else {continue}
            }
            let angle_name = format!("{}", a); // rust Y U NO eq f32
            if let Some(aligned) = targets.get_mut(&angle_name) {
                aligned.sort_by(|a, b| distance(*a.0, *a.1, *b.0, *b.1, &base));
                let pow = aligned.remove(0);
                count += 1;
                println!("{} : fuck THAT rock {:?} \tangle {}", count, pow, angle_name);
                if aligned.len() > 0 {
                    println!("   missed the other {:?}", aligned[0]);
                }
                if aligned.is_empty() {
                    targets.remove(&angle_name);
                }
            }
            if count == 200 {break 'kill}
        }
    }

    Ok(())
}

fn distance(ax: isize, ay: isize, bx: isize, by: isize, base: &(isize, isize)) -> Ordering {
    let ad = (((ax - base.0).pow(2) + (ay - base.1).pow(2)) as f32).sqrt();
    let bd = (((bx - base.0).pow(2) + (by - base.1).pow(2)) as f32).sqrt();
    if ad > bd {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}

fn spiral_out(rayon: usize) -> Vec<(isize, isize)> {
    let area = (rayon * 2).pow(2);
    let mut seg_len = 2;
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
                if points.len() > area {break 'out}
            }
        }
        y += 1;
        x -= 1;
        seg_len += 2;
    }
    points
}
