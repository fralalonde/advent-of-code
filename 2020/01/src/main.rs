use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;
use std::iter::Enumerate;

fn main() -> io::Result<()> {
    let f = File::open("01/input")?;
    let file = BufReader::new(&f);
    let nums: Vec<u32> = file.lines().into_iter().map(|s| match s {
        Ok(s) => s.parse::<u32>().unwrap(),
        _ => 0,
    })
    .collect();

    'gg:
    for (z, i) in nums.iter().enumerate() {
        let subs = &nums[z..];
        for j in subs {
            if i + j == 2020 {
                println!("found it {:?}", i * j);
                break 'gg
            }
        }
    }

    'zz:
    for (z, i) in nums.iter().enumerate() {
        let subs = &nums[z..];
        for (y, j) in subs.iter().enumerate() {
            let subs = &nums[y..];
            for k in subs {
                if i + j + k == 2020 {
                    println!("found it {:?}", i * j * k);
                    break 'zz
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
}
