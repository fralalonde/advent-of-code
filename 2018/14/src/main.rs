use std::io;
use std::env;

fn print(prefix: &str, slice: &[usize]) {
    print!("{} ", prefix);
    for c in slice {
        print!("{}", c);
    }
    println!();
}

fn push_check_len(target: usize, scores: &mut Vec<usize>, first: &mut Option<usize>, i: usize) -> bool {
    scores.push(i);
    if let Some(frrr) = first {
        if (scores.len() - *frrr) > 10 {
            print("part a: ", &scores[*frrr..*frrr + 10]);
            return false
        }
    }
    if scores.len() == target {
        *first = Some(scores.len())
    }
    true
}

fn push_check_mask(target_mask: &[usize], scores: &mut Vec<usize>, i: usize) -> bool {
    scores.push(i);
    if scores.ends_with(target_mask) {
        println!("part b: {}", scores.len() - 5);
        false
    } else {
        true
    }
}

fn main() -> io::Result<()> {
    let mut rec_num = [3, 7];
    let mut rec_idx = [0, 1];
    let mut scores: Vec<usize> = vec![3, 7];
    let mut first_recipe = None;

    let target_len = env::args().skip(1).next().expect("recipe step count").parse::<usize>().expect("valid step count");

    // part a
    loop {
       let sum = rec_num[0] + rec_num[1];
        if sum > 9 {
            if !push_check_len(target_len, &mut scores, &mut first_recipe, sum / 10) {break};
            if !push_check_len(target_len, &mut scores, &mut first_recipe, sum % 10) {break};
        } else {
            if !push_check_len(target_len, &mut scores, &mut first_recipe, sum) {break};
        }
        for elf in 0..rec_num.len() {
            let fwd = rec_num[elf] + 1;
            rec_idx[elf] = (rec_idx[elf] + fwd) % scores.len();
            rec_num[elf] = scores[rec_idx[elf]];
        }
    }

    // part b

    let mut target_mask = vec![];
    let mut tdec = target_len;
    loop {
        target_mask.push(tdec % 10);
        tdec = tdec / 10;
        if tdec < 1 {break}
    }
    target_mask.reverse();

    print("mask ", &target_mask);


    let mut rec_num = [3, 7];
    let mut rec_idx = [0, 1];
    let mut scores: Vec<usize> = vec![3, 7];

    loop {
        let sum = rec_num[0] + rec_num[1];
        if sum > 9 {
            if !push_check_mask(&target_mask, &mut scores, sum / 10) {break};
            if !push_check_mask(&target_mask, &mut scores, sum % 10) {break};
        } else {
            if !push_check_mask(&target_mask, &mut scores, sum) {break};
        }
        for elf in 0..rec_num.len() {
            let fwd = rec_num[elf] + 1;
            rec_idx[elf] = (rec_idx[elf] + fwd) % scores.len();
            rec_num[elf] = scores[rec_idx[elf]];
        }
    }

    Ok(())
}
