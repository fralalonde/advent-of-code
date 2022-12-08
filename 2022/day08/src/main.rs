use std::fs::File;
use std::io::{BufRead, BufReader};

fn forest(buf: impl BufRead) -> Vec<Vec<u8>> {
    buf.lines().filter_map(|x| x.ok()).into_iter()
        .map(|x| x.into_bytes().into_iter().map(|x| x - 48).collect())
        .collect()
}

fn vis(forets: &Vec<Vec<u8>>, x: i32, y: i32) -> bool {
    let udlr = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let top = forets[x as usize][y as usize];
    for d in &udlr {
        let mut x1 = x;
        let mut y1 = y;

        loop {
            x1 += d.0;
            y1 += d.1;
            if x1 < 0 || x1 >= forets[0].len() as i32 || y1 < 0 || y1 >= forets.len() as i32 {
                return true;
            }
            if forets[x1 as usize][y1 as usize] >= top { break; }
        }
    }
    false
}

fn score(forets: &Vec<Vec<u8>>, x: i32, y: i32) -> i32 {
    let udlr = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut score = vec![];
    let top = forets[x as usize][y as usize];

    for d in &udlr {
        let mut x1 = x;
        let mut y1 = y;
        let mut dist = 0;

        loop {
            x1 += d.0;
            y1 += d.1;
            if x1 < 0 || x1 >= forets[0].len() as i32 || y1 < 0 || y1 >= forets.len() as i32 {
                score.push(dist);
                break;
            }
            dist += 1;
            if forets[x1 as usize][y1 as usize] >= top {
                score.push(dist);
                break;
            }
        }
    }
    score[0] * score[1] * score[2] * score[3]
}

fn main() -> anyhow::Result<()> {
    let f = File::open("day08/input")?;
    let file = BufReader::new(&f);

    let f = forest(file);
    let mut visc = 0;
    for x in 0..f[0].len() {
        for y in 0..f.len() {
            if vis(&f, x as i32, y as i32) { visc += 1 }
        }
    }
    println!("counrt {:?}", visc);

    let mut max = 0;
    for x in 0..f[0].len() {
        for y in 0..f.len() {
            max = i32::max(score(&f, x as i32, y as i32), max)
        }
    }
    println!("marx {:?}", max);

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use std::io::BufReader;

    static INPUT: &str = r"30373
25512
65332
33549
35390";

    #[test]
    fn ting() -> anyhow::Result<()> {
        assert_eq!(true, false);
        Ok(())
    }
}