extern crate regex;
extern crate linked_list;

use std::fs::File;
use std::io::{BufReader, self, Read, BufRead,};
use linked_list::{LinkedList, Cursor};
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::{max};
use std::cell::RefCell;
use regex::Regex;
use std::cmp::Ordering;
use std::thread::sleep_ms;

const FILENAME: &'static str = "input";
//const FILENAME: &'static str = "test";

struct Node {
    nodes: Vec<Node>,
    meta:  Vec<usize>,
}

struct Point {
    pos_x: isize,
    pos_y: isize,
    vel_x: isize,
    vel_y: isize,
}

fn main() -> io::Result<()> {
    let re = regex::Regex::new("position=<\\s*([-\\d]+),\\s*([-\\d]+)>\\s*velocity=<\\s*([-\\d]+),\\s*([-\\d]+)>").unwrap();
    let f = File::open(FILENAME)?;
    let mut file = BufReader::new(&f);
    let mut points: Vec<Point> = file.lines()
        .filter_map(|x| x.ok())
        .map(|s| {
            let caps = re.captures(&s).unwrap();
            Point {
                pos_x: caps.get(1).unwrap().as_str().parse::<isize>().unwrap(),
                pos_y: caps.get(2).unwrap().as_str().parse::<isize>().unwrap(),
                vel_x: caps.get(3).unwrap().as_str().parse::<isize>().unwrap(),
                vel_y: caps.get(4).unwrap().as_str().parse::<isize>().unwrap(),
            }
        })
        .collect();

    let mut sec = 0;

    loop {
        sec += 1;
        points.iter_mut().for_each(|ref mut p| {p.pos_x += p.vel_x; p.pos_y += p.vel_y});

        points.sort_by(|p1, p2| match p1.pos_y.cmp(&p2.pos_y) {
            Ordering::Equal => p1.pos_x.cmp(&p2.pos_x),
            z => z,
        });

        let min_x = points.iter().map(|p| p.pos_x).min().unwrap();
        let max_x = points.iter().map(|p| p.pos_x).max().unwrap();

        if max_x - min_x > (points.len() as isize + 6) {
            continue;
        }

        let mut y = points[0].pos_y;
        let mut x = min_x;
        let mut str1 = String::new();

        print!("{}[2J", 27 as char);

        for p in &points {
//            if (p.pos_x - min_x) > 120 { continue }
            if y != p.pos_y {
                y = p.pos_y;
                x = min_x;
                str1.push_str("\n");
            }
            if x != p.pos_x {
                for _ in x..(p.pos_x - 1) {
                    str1.push_str(" ");
                }
                str1.push_str("#");
                x = p.pos_x;
            }
        }
        println!("{}", str1);
        println!("\n-------------------sec {}-----min_x {} max {}  points[0].pos_y {};\n", sec, min_x, max_x, points[0].pos_y);
        str1.clear();
        sleep_ms(600);

    }

    Ok(())
}



