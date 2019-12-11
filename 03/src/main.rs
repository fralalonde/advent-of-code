use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Line {
    x: isize,
    y: isize,
    horizontal: bool,
    len: isize,
}

impl Line {
    fn in_range(&self, z: isize) -> bool {
        if self.horizontal {
            if self.len > 0 {
                z >= self.x && z <= self.x + self.len
            } else {
                z >= self.x + self.len && z <= self.x
            }
        } else {
            if self.len > 0 {
                z >= self.y && z <= self.y + self.len
            } else {
                z >= self.y + self.len && z <= self.y
            }
        }
    }

    fn intersect(&self, other: &Line) -> Option<(isize, isize)> {
        if self.horizontal == other.horizontal {
            return None;
        }
        if self.horizontal {
            if other.in_range(self.y) && self.in_range(other.x) {
                return Some((other.x, self.y));
            }
        } else {
            if other.in_range(self.x) && self.in_range(other.y) {
                return Some((self.x, other.y));
            }
        }
        None
    }

    fn end(&self) -> (isize, isize) {
        if self.horizontal {
            (self.x + self.len, self.y)
        } else {
            (self.x, self.y + self.len)
        }
    }
}

fn main() -> Result<()> {
    let f = File::open("03/input")?;
    let file = BufReader::new(&f);
    let wires: Vec<String> = file.lines().into_iter().filter_map(|x| x.ok()).collect();

    let mut lines: Vec<Vec<Line>> = vec![];
    for wire in wires {
        let mut start = (0, 0);
        let mut l = vec![];
        let zz: Vec<&str> = wire.split(",").collect();
        for w in zz {
            let (d, len) = w.split_at(1);
            let len = len.parse::<isize>()?;
            let (horizontal, len) = match d {
                "U" => (false, len),
                "D" => (false, -len),
                "R" => (true, len),
                "L" => (true, -len),
                _ => return Err(anyhow!("fuck")),
            };
            let mew = Line {
                x: start.0,
                y: start.1,
                horizontal,
                len,
            };
            println!("({}, {})", mew.end().0, mew.end().1);
            start = mew.end();
            l.push(mew)
        }
        lines.push(l);
    }

    // part 1
    let mut dist = std::isize::MAX;
    let mut minsteps = std::isize::MAX;

    let mut asteps = 0;
    for a in &lines[0] {
        let mut bsteps = 0;
        for b in &lines[1] {
            if let Some((x, y)) = b.intersect(&a) {
                if x == 0 && y == 0 {
                    continue;
                }
                println!("{:?} / {:?} \n @ {}, {}", b, a, x, y);
                let d = x.abs() + y.abs();
                if dist > d {
                    dist = d
                }
                let (ai, bi) = if a.horizontal {
                    ((a.x - x).abs(), (b.y - y).abs())
                } else {
                    ((b.x - x).abs(), (a.y - y).abs())
                };
                let s = bsteps + asteps + ai + bi;
                if s < minsteps {
                    minsteps = s;
                }
            }
            bsteps += b.len.abs();
        }
        asteps += a.len.abs();
    }

    println!("min dist {}", dist);
    println!("min steps {}", minsteps);

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::Line;

    #[test]
    fn intersects() {
        let a = Line {
            x: 0,
            y: 0,
            horizontal: true,
            len: 10,
        };
        let b = Line {
            x: 5,
            y: 5,
            horizontal: false,
            len: -10,
        };
        assert_eq!(a.intersect(&b), Some((5, 0)));
        assert_eq!(b.intersect(&a), Some((5, 0)));

        let c = Line {
            x: 20,
            y: 20,
            horizontal: false,
            len: 40,
        };
        assert_eq!(a.intersect(&c), None);
        assert_eq!(b.intersect(&c), None);

        let d = Line {
            x: 21,
            y: 19,
            horizontal: true,
            len: -3,
        };
        assert_eq!(c.intersect(&d), Some((20, 19)));
        assert_eq!(d.intersect(&c), Some((20, 19)));
    }
}
