use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;
use std::str::FromStr;

fn main() -> io::Result<()> {
    let f = File::open("day04/input")?;
    let file = BufReader::new(&f);

    let mut full = 0;
    let mut part = 0;
    for line in file.lines().into_iter().filter_map(|l| l.ok()) {
        let z = line.split(",");
        let w: Vec<_> = z.flat_map(|r| r.split("-")).map(|i| usize::from_str(i).unwrap()).collect();
        let range_a = w[0]..=w[1];
        let range_b = w[2]..=w[3];
        if (range_a.contains(&w[2]) && range_a.contains(&w[3]))
            || (range_b.contains(&w[0]) && range_b.contains(&w[1]))
        {
            full += 1;
        }
        if (range_a.contains(&w[2]) || range_a.contains(&w[3]))
            || (range_b.contains(&w[0]) || range_b.contains(&w[1]))
        {
            part += 1;
        }
    }

    println!("score A {:?}", full);
    println!("score B {:?}", part);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn ting() {
        assert_eq!(0, 0);
    }
}