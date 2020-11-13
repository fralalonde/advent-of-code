extern crate regex;

use std::fs::File;
use std::io::{BufReader, BufRead, self};
use regex::Regex;
use std::rc::Rc;
use std::collections::{HashSet};
use std::cmp::{max, min};
use std::cmp::Ordering;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Claim {
    id: usize,
    x: usize,
    w: usize,
    y: usize,
    h: usize,
}

fn main() -> io::Result<()> {

    let re = Regex::new("#(\\d+) @ (\\d+),(\\d+): (\\d+)x(\\d+)").unwrap();

    let f = File::open("input")?;
    let file = BufReader::new(&f);
    let claims: Vec<Rc<Claim>> = file.lines()
        .filter_map(|x| x.ok())
        .map(|s| {
            let caps = re.captures(&s).unwrap();
            Rc::new(Claim {
                id: caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                x: caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                y: caps.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                w: caps.get(4).unwrap().as_str().parse::<usize>().unwrap(),
                h: caps.get(5).unwrap().as_str().parse::<usize>().unwrap(),
            })
        })
        .collect();

    // sort claims horizontally, by position first and width second
    let mut hsort = claims.to_vec().clone();
    hsort.sort_by(|a, b|
        match (a, b) {
            (a, b) if a.x > b.x => Ordering::Greater,
            (a, b) if a.x < b.x => Ordering::Less,
            (a, b) if a.w > b.w => Ordering::Greater,
            (a, b) if a.w < b.w => Ordering::Less,
            _ => Ordering::Equal,
        }
    );

    // pool to remove dirty claims from for part 2
    let mut clean_claims = HashSet::new();
    claims.iter().for_each(|c| { clean_claims.insert(c.clone()); });

    // pool of duplicate pixels ("square inches")
    let mut pixels: HashSet<(usize, usize)> = HashSet::new();

    // iterate horizontally
    for (idx, claim) in hsort.iter().enumerate() {
        let claim_hend = claim.x + claim.w;
        let claim_vend = claim.y + claim.h;
        let (_prev, next) = hsort.split_at(idx + 1);
        for c2 in next {
            // no more horizontally overlapping claims for this one?
            if c2.x > claim_hend { break }

            // check for vertical overlap
            if c2.y > claim_vend { continue }
            let c2_vend = c2.y + c2.h;
            if claim.y > c2_vend { continue }

            let rhend = min(c2.x + c2.w, claim_hend);
            let ry = max(claim.y, c2.y);
            let rvend = min(c2_vend, claim_vend);

//            println!("{:?}\n{:?}", claim, c2);
//            println!("{} {} {} {}", c2.x, rhend, ry, rvend);

            // overlap detected, remove dirty claims
            clean_claims.remove(c2);
            clean_claims.remove(claim);

            // mark individual pixels of overlapping region
            for f in c2.x..rhend {
//                print!("x: {} ", f);
                for g in ry..rvend {
//                    print!(" {}", g);
                    pixels.insert((f, g));
                }
//                println!();
            }
        }
    }

    println!("dup pixel count {:?}", pixels.len());

    println!("remaining claim {:?}", clean_claims);

    Ok(())
}
