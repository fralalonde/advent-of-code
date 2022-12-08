use std::fs::File;
use std::io::{Read};


fn main() -> anyhow::Result<()> {
    let mut f = File::open("day06/input")?;
    let mut line = vec![];

    f.read_to_end(&mut line)?;
    println!("end of marker {:?}", distinct(4, &line));
    println!("end of marker {:?}", distinct(14, &line));

    Ok(())
}

fn distinct(reqlen: usize, line: &[u8]) -> Option<usize> {
    let mut wstart = 0;
    let mut wend = 1;

    loop {
        let mut i = wend - 1;
        loop {
            if wend == line.len() {
                return None;
            }
            if line[i] == line[wend] {
                wstart = i;
                break;
            }
            if (wend - wstart) > reqlen {
                return Some(wend);
            }
            if i > wstart {
                i -= 1;
            } else {
                break;
            }
        }
        wend += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::distinct;

    #[test]
    fn ting() {
        assert_eq!(Some(5), distinct(4, "bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes()));
        assert_eq!(Some(6), distinct(4, "nppdvjthqldpwncqszvftbrmjlhg".as_bytes()));
        assert_eq!(Some(10), distinct(4, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes()));
        assert_eq!(Some(11), distinct(4, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes()));
    }
}