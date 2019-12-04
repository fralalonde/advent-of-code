use std::io;
use std::fs::File;
use std::io::{BufReader, BufRead};
use anyhow::*;
use std::ops::Range;
use std::cmp::max;

/// returns true if max has been reached
fn lap(og: &[usize], max_v: usize, mut d: usize, pre_v: usize, pre_i: usize, seq: bool, comb: &mut usize) -> bool {
    let start = max(pre_i, og[d - 1]);
    let seq = seq || pre_i == start;
    let pre_v = pre_v * 10;
    for i in start..10 {
        let z = pre_v + i;
        if d == og.len() {
            if seq {
                println!("{}", z);
                *comb += 1
            }
            if z >= max_v { return true };
        } else {
            if lap(og, max_v, d + 1, z, i, seq, comb) { return true }
        }
    }
    false
}

fn count(og: &[usize], max_v: usize) -> usize {
    let mut comb: usize = 0;
    lap(og, max_v, 1, 0, 0, false, &mut comb);
    comb
}

fn main() -> Result<()> {

    let c = count(&[3,0,7,2,3,7], 769058);

    println!("comb {}", c);

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::count;

    #[test]
    fn chin() {
        let c = count(&[0, 0], 99);
        println!("comb {}", c);
    }
}
