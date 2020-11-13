extern crate regex;

use std::fs::File;
use std::io::{BufReader, BufRead, self};
use regex::{Regex, Captures};
use std::rc::Rc;
use std::collections::{HashMap, HashSet};
use std::cmp::{max, min};
use std::cmp::Ordering;

type Date = usize;

type Minute = usize;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Entry (Date, Minute, Event);

#[derive(Debug, Hash, Eq, PartialEq)]
enum Event {
    /// guard id
    Shift(usize),

    /// minute
    Asleep,

    /// minute
    Awake,
}

fn cap_usize(caps: &Captures, idx: usize) -> usize {
    caps.get(idx).unwrap().as_str().parse::<usize>().unwrap()
}

fn make_date(caps: &Captures) -> Date {
    (cap_usize(caps, 1) * 1000000) +
        (cap_usize(caps, 2) * 10000) +
        (cap_usize(caps, 3) * 100) +
        (cap_usize(caps, 4))
}

fn main() -> io::Result<()> {

    let guard_ex = Regex::new(r"\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] Guard #(\d+) begins shift").unwrap();
    let sleep_ex = Regex::new(r"\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] falls asleep").unwrap();
    let awake_ex = Regex::new(r"\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] wakes up").unwrap();

    let f = File::open("input")?;
    let file = BufReader::new(&f);
    let mut log: Vec<Entry> = file.lines().into_iter()
        .filter_map(|x| x.ok())
        .map(|s| match s.split_at(s.len() - 2) {
            (_, "ft") => {
                let caps = guard_ex.captures(&s).unwrap();
                Entry(make_date(&caps), cap_usize(&caps, 5), Event::Shift(cap_usize(&caps, 6)))
            },
            (_, "ep") => {
                let caps = sleep_ex.captures(&s).unwrap();
                Entry(make_date(&caps), cap_usize(&caps, 5), Event::Asleep)
            },
            (_, "up") => {
                let caps = awake_ex.captures(&s).unwrap();
                Entry(make_date(&caps), cap_usize(&caps, 5), Event::Awake)
            },
            _ => panic!("WUUUU")
        })
        .collect();

    log.sort_by(|a, b| {
        if a.0 > b.0 {
            Ordering::Greater
        } else if a.0 < b.0 {
            Ordering::Less
        } else {
            a.1.cmp(&b.1)
        }
    });

//    for l in &log {
//        println!("entry {:?}", l);
//    }

    // (guard, minute), freq_count
    let mut minutes: HashMap<(usize, usize), usize> = HashMap::new();
    let mut guard_m: HashMap<usize, usize> = HashMap::new();

    let mut i = log.iter();
    let mut guard = 0;
    let mut asleep = 0;
    loop {
        match i.next() {
            Some(entry) => {
                match entry {
                    Entry(_date, _minute, Event::Shift(id)) => guard = *id,
                    Entry(_date, minute, Event::Asleep) => asleep = *minute,
                    Entry(_date, awake, Event::Awake) => {
                        *guard_m.entry(guard).or_insert(0) += *awake - asleep;
                        for m in asleep..*awake {
                            *minutes.entry((guard, m)).or_insert(0) += 1;
                        }
                    },
                }
            }
            None => break,
        }
    }

    let mut g2: Vec<(usize, usize)> = vec![];
    for (k, v) in guard_m.into_iter() {
        g2.push((k, v))
    }

    g2.sort_by(|a, b| b.1.cmp(&a.1));

//    for g in &g2 {
//        let z = m.0;
//        println!("guard {:?}", g);
//    }

    let mut m2: Vec<((usize, usize), usize)> = vec![];
    let mut m3: Vec<((usize, usize), usize)> = vec![];

    for (k, v) in minutes.into_iter() {
        if k.0 == g2[0].0 {
            m2.push((k, v))
        }
        m3.push((k, v))
    }

    let value_sort = |a: &(_, usize), b: &(_, usize)| -> Ordering {
        b.1.cmp(&a.1)
    };

    m2.sort_by(value_sort);
    m3.sort_by(value_sort);

//    for m in &m2 {
//        let z = m.0;
//        println!("minute {:?} => {}", m, z.0 * z.1);
//    }

    let z3 = m3[0].0;
    println!("{:?} => {}", z3, z3.0 * z3.1);

    let z2 = m2[0].0;
    println!("{:?} => {}", z2, z2.0 * z2.1);

    Ok(())
}
