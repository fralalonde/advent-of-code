use anyhow::*;

/// returns true if max has been reached
fn lap(
    og: &mut [usize],
    max_v: usize,
    d: usize,
    pre_v: usize,
    pre_i: usize,
    pseq: bool,
    comb: &mut usize,
) -> bool {
    let start = if pre_i > og[d] {
        for z in d..og.len() {
            og[z] = pre_i;
        }
        pre_i
    } else {
        og[d]
    };
    let pre_v = pre_v * 10;
    for i in start..10 {
        let z = pre_v + i;
        if z >= max_v {
            return true;
        };
        let seq = pseq || pre_i == i;
        if d == og.len() - 1 {
            if seq {
                *comb += 1
            }
        } else {
            if lap(og, max_v, d + 1, z, i, seq, comb) {
                return true;
            }
        }
    }
    false
}

fn lop(
    og: &mut [usize],
    max_v: usize,
    d: usize,
    pre_v: usize,
    pre_i: usize,
    pseq: bool,
    comb: &mut usize,
) -> bool {
    let start = if pre_i > og[d] {
        for z in d..og.len() {
            og[z] = pre_i;
        }
        pre_i
    } else {
        og[d]
    };
    let pre_v = pre_v * 10;
    for i in start..10 {
        let z = pre_v + i;
        if z >= max_v {
            return true;
        };
        let seq = (pre_i == i) && !pseq;
        if d == og.len() - 1 {
            if seq {
                println!("{}", z);
                *comb += 1
            }
        } else {
            if lap(og, max_v, d + 1, z, i, seq, comb) {
                return true;
            }
        }
    }
    false
}

fn count(og: &mut [usize], max_v: usize) -> usize {
    let mut comb: usize = 0;
    lap(og, max_v, 0, 0, 0, false, &mut comb);
    comb
}

fn count2(og: &mut [usize], max_v: usize) -> usize {
    let mut comb: usize = 0;
    lop(og, max_v, 0, 0, 0, false, &mut comb);
    comb
}

fn main() -> Result<()> {
    let c = count(&mut [3, 0, 7, 2, 3, 7], 769058);

    println!("comb {}", c);

    let c = count2(&mut [3, 0, 7, 2, 3, 7], 769058);

    println!("partb {}", c);

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::count;

    #[test]
    fn chin() {
        let c = count(&mut [0, 0], 99);
        println!("comb {}", c);

        let c = count(&mut [5, 3, 2], 874);
        println!("comb {}", c);

        let c = count(&mut [5, 3, 2], 421);
        println!("comb {}", c);

        let c = count(&mut [5, 3], 89);
        println!("comb {}", c);
    }
}
