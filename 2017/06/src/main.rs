fn max_idx(a: &[u8]) -> usize {
    let mut max = a[0];
    let mut idx = 0;
    for i in 1..a.len() {
        if a[i] > max {
            max = a[i];
            idx = i;
        }
    }
    idx
}

use std::collections::HashSet;
//
//fn count(z: Vec<u8>) -> usize {
//
//}

fn main() {
    let mut z = vec![14, 0, 15, 12, 11, 11, 3, 5, 1, 6, 8, 4, 9, 1, 8, 4];
    let mut seen = HashSet::new();
    while seen.insert(z.clone()) {
        let mut idx = max_idx(&z);
        let mut dist = z[idx];
        z[idx] = 0;
        while dist > 0 {
            idx += 1;
            if idx >= z.len() {
                idx = 0;
            }
            z[idx] += 1;
            dist -= 1;
        }
    }

    println!("seen count {}", seen.len());

    seen.clear();
    while seen.insert(z.clone()) {
        let mut idx = max_idx(&z);
        let mut dist = z[idx];
        z[idx] = 0;
        while dist > 0 {
            idx += 1;
            if idx >= z.len() {
                idx = 0;
            }
            z[idx] += 1;
            dist -= 1;
        }
    }
    println!("cycle count {}", seen.len());
}

//#[cfg(test)]
//pub fn test1() {
//    assert_eq!(5, count(vec![0,2,7,0]));
//}