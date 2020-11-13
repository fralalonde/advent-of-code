extern crate regex;

use std::fs::File;
use std::io::{BufReader, BufRead, self};
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

use regex::Regex;

use self::Direction::*;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Coord {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Coord {
    fn from(c: (i32, i32)) -> Self {
        Coord {
            x: c.0,
            y: c.1,
        }
    }
}

/// inclusive bounds rectangle
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Region {
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
}

impl Region {
    fn extend(&mut self, dir: Direction, dist: i32) {
        match dir {
            Left => self.left -= dist,
            Right => self.right += dist,
            Up => self.top -= dist,
            Down => self.bottom += dist,
        }
    }
}

impl <'a> From<&'a Coord> for Region {
    /// build minimal region of size 1
    fn from(coord: &'a Coord) -> Self {
        Region {
            left: coord.x,
            right: coord.x,
            top: coord.y,
            bottom: coord.y,
        }
    }
}

impl Coord {
    fn is_within(&self, region: &Region) -> bool {
        self.x <= region.right
            && self.x >= region.left
            && self.y <= region.bottom
            && self.y >= region.top
    }

    fn distance_to(&self, coord: &Coord) -> usize {
        ((self.x - coord.x).abs() + (self.y - coord.y).abs()) as usize
    }

    fn step(&mut self, d: Direction) {
        match d {
            Left => self.x -= 1,
            Right => self.x += 1,
            Up => self.y -= 1,
            Down => self.y += 1,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn main() -> io::Result<()> {
    let re = Regex::new("(\\d+), (\\d+)").unwrap();

    let f = File::open("input")?;
    let file = BufReader::new(&f);
    let coords: Vec<Rc<Coord>> = file.lines()
        .filter_map(|x| x.ok())
        .map(|s| {
            let caps = re.captures(&s).unwrap();
            Rc::new(Coord {
                x: caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                y: caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            })
        })
        .collect();

    part_a(&coords);
    part_b(&coords);
    Ok(())
}

fn part_a(coords: &Vec<Rc<Coord>>) {
    let universe = Region {
        left: coords.iter().map(|a| a.x).min().unwrap(),
        right: coords.iter().map(|a| a.x).max().unwrap(),
        top: coords.iter().map(|a| a.y).min().unwrap(),
        bottom: coords.iter().map(|a| a.y).max().unwrap(),
    };
    
    let mut enclosed_region = HashMap::new();

    // this scan is very slow and could be sped up by building a lookup cache
    // ...which was there at some point but not required for the amount of input we have
    'coord: for origin in coords {
        // walk in every direction to try and find bounds
        let mut region = Region::from(origin.as_ref());
        for direction in &[Left, Right, Up, Down,] {
            let mut cursor: Coord = origin.as_ref().clone();
            let mut origin_dist = 0;
            'direction: loop {
                origin_dist += 1;
                cursor.step(*direction);
                if !cursor.is_within(&universe) {
                    // escaped!
                    continue 'coord;
                }
                for neighbor in coords {
                    if neighbor != origin {
                        if neighbor.distance_to(&cursor) <= origin_dist {
                            // blocked by other coord
                            break 'direction;
                        }
                    }
                }
            }
            region.extend(*direction, origin_dist as i32 - 1);
        }
        enclosed_region.insert(origin.clone(), region);
    }

    let mut owners_pixel = HashMap::new();

    // process pixels from enclosed regions
    for (_origin, region) in &enclosed_region {
        for x in region.left..=region.right {
            for y in region.top..=region.bottom {
                let here: Coord = (x, y).into();
                let mut distance: Vec<(Rc<Coord>, usize)> = coords.iter().map(|c| (c.clone(), c.distance_to(&here))).collect();
                distance.sort_by(|a, b| a.1.cmp(&b.1));
                // check if equidistant no man's land
                if distance[0].1 != distance[1].1 {
                    if enclosed_region.contains_key(&distance[0].0) {
                        // mark owner
                        owners_pixel.entry(distance[0].0.clone()).or_insert(HashSet::new()).insert(here);
                    }
                }
            }
        }
    }

    let mut sorted_areas: Vec<(Rc<Coord>, usize)> = owners_pixel.into_iter().map(|(k, v)| (k, v.len())).collect();
    sorted_areas.sort_by(|a, b| b.1.cmp(&a.1));
    println!("{:?} area {:?}", sorted_areas[0].0, sorted_areas[0].1);
}


fn part_b(coords: &Vec<Rc<Coord>>) {
    let universe = Region {
        left: coords.iter().map(|a| a.x).min().unwrap(),
        right: coords.iter().map(|a| a.x).max().unwrap(),
        top: coords.iter().map(|a| a.y).min().unwrap(),
        bottom: coords.iter().map(|a| a.y).max().unwrap(),
    };

    let mut within = 0;

    // process pixels from enclosed regions
    for x in universe.left..=universe.right {
        for y in universe.top..=universe.bottom {
            let here: Coord = (x, y).into();
            let distance: usize = coords.iter().map(|c| c.distance_to(&here)).sum();
            if distance < 10000 {
                within += 1;
            }
        }
    }

    println!("area {:?}", within);
}

