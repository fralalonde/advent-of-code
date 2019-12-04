use std::io;
use std::fs::File;
use std::io::{BufReader, BufRead};
use anyhow::*;
use std::ops::Range;
use std::cmp::max;

const MAX_DEPTH: usize = 5;

//const MAX_VAL: usize = 769058;
//const OG: [usize; 6] = [3,0,7,2,3,7];

const MAX_VAL: usize = 888889;
const OG: [usize; 6] = [8,8,8,8,8,8];

/// returns true if max has been reached
fn lap(mut d: usize, pre_v: usize, pre_i: usize, seq: bool, comb: &mut usize) -> bool {
    let start = max(pre_i, OG[d]);
    let seq = seq || pre_i == start;
    for i in start..10 {
        match d {
            MAX_DEPTH => {
                if !seq { return false }
                if pre_v + i > MAX_VAL { return true };
                *comb += 1;
            }
            _ => if lap(d + 1, pre_v * 10  + i, i, seq, comb) { return true }
        }
    }
    false
}


fn main() -> Result<()> {

    let mut comb: usize = 0;
    lap(0, 0, 0, false, &mut comb);

    println!("comb {}", comb);

    Ok(())
}

#[cfg(test)]
mod test {

    #[test]
    fn bop() {
    }
}
