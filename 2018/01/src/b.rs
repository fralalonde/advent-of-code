use std::collections::HashSet;

pub fn run(lines: &[i32]) {
    let mut set = HashSet::new();
    let mut sum = 0;

    'main: loop {
        for i in lines {
            sum += i;
            if !set.insert(sum) { break 'main }
        }
    }
    println!("dupfreq {}", sum)
}
