extern crate regex;

use std::fs::File;
use std::io::{BufReader, BufRead, self};
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::{max};
use std::cell::RefCell;
use regex::Regex;
use std::cmp::Ordering;

// test values
//const ASCII_TIME_OFFSET: u8 = 64;
//const WORKERS: usize = 2;

// input value
const ASCII_TIME_OFFSET: u8 = 4;
const WORKERS: usize = 5;
const FILENAME: &'static str = "input";

fn main() -> io::Result<()> {
    let re = Regex::new("Step (.) must be finished before step (.) can begin.").unwrap();

    let f = File::open(FILENAME)?;
    let file = BufReader::new(&f);
    let pairs: Vec<(String, String)> = file.lines()
        .filter_map(|x| x.ok())
        .map(|s| {
            let caps = re.captures(&s).unwrap();
            (caps.get(1).unwrap().as_str().to_owned(), caps.get(2).unwrap().as_str().to_owned())
        })
        .collect();

    let mut reqs = HashMap::new();
    let mut heed = HashSet::new();
    for p in &pairs {
        heed.insert(p.1.clone());
        heed.insert(p.0.clone());
        reqs.entry(p.1.clone()).or_insert(vec![]).push(p.0.clone());
    }

    let mut head: Vec<String> = heed.into_iter().collect();
    head.sort();

    println!("{:?}", head);

    let pool: RefCell<[(u8, usize); WORKERS]> = RefCell::new([(0,0); WORKERS]);
    let mut done = HashMap::new();

    loop {
        if !head.iter().any(|h| walk(h, &mut done, &reqs, &pool)) {
            break;
        }
    }

    let mut all: Vec<(String, usize)> = done.into_iter().collect();
    all.sort_by(|a, b| match a.1.cmp(&b.1) {
        Ordering::Equal => a.0.cmp(&b.0),
        z => z,
    });
    all.iter().for_each(|s| print!("{}", s.0));

    let max_end_time = pool.borrow().iter().map(|x| x.1).max().unwrap();

    println!("\ntime {}", max_end_time);

    Ok(())
}

fn walk(node: &String, done: &mut HashMap<String, usize>, revv: &HashMap<String, Vec<String>>, pool: &RefCell<[(u8, usize); WORKERS]>) -> bool {
    if done.get(node).is_some() {
        return false;
    }

    let mut min_start_time = 0;
    if let Some(prereq) = revv.get(node) {
        for w in prereq {
            if let Some(req_end_time) = done.get(w) {
                min_start_time = max(min_start_time, *req_end_time);
            } else {
                return false;
            }
        }
    }

    let duration = node.as_bytes()[0] - ASCII_TIME_OFFSET;
    let (idx, delay) = pool.borrow().iter().enumerate().min_by_key(|(_idx, delay)| delay.0).map(|(idx, delay)| (idx, delay.clone())).unwrap();

    for i in 0..WORKERS {
        pool.borrow_mut()[i].0 -= delay.0;
    }

    let real_start_time = max(min_start_time, delay.1);
    let end_time = real_start_time + duration as usize;

    pool.borrow_mut()[idx] = (duration, end_time);
    done.insert(node.clone(), end_time);

    true
}


