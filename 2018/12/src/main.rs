extern crate regex;

use std::fs::File;
use std::io::{BufReader, self, Read, BufRead,};
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::{max};
use std::cell::RefCell;
use regex::Regex;
use std::cmp::Ordering;
use std::thread::sleep_ms;
use std::rc::Rc;

fn main() -> io::Result<()> {
    let mut offset = 0;

    let mut buf = "##.####..####...#.####..##.#..##..#####.##.#..#...#.###.###....####.###...##..#...##.#.#...##.##..".to_string();

    for z in 1..=20 {
        let z = gen(offset, &buf);
        offset = z.0;
        buf = z.1;
    }

    let mut sum = 0;
    for i in 0..buf.len() {
        if &buf[i..i+1] == "#" {
            let real = i as isize + offset;
            sum += real;
        }
    }

    println!("sum {}", sum);

    // part 2

    // pattern stabilizes after ~125 iterations
    for z in 1..=200 {
        if z % 10 == 0 {
            println!("{}: offset ({}) {}", z, offset, buf);
        }
        let z = gen(offset, &buf);
        offset = z.0;
        buf = z.1;
    }

    let delta = 200 - offset as u64;
    let big_offset = 50_000_000_000 - delta;

    let mut big_sum: u64 = 0;
    for i in 0..buf.len() {
        if &buf[i..i+1] == "#" {
            let real = i as u64 + big_offset;
            big_sum += real;
        }
    }

    println!("{:?}: offset ({}) {}", 50_000_000_000, big_offset, buf);

    println!("big sum {}", big_sum);

    Ok(())
}

fn gen(mut offset: isize, buffer: &str) -> (isize, String) {
    let mut prev_buff = String::new();

    if buffer.starts_with("#") {
        offset -= 3;
        prev_buff.push_str("...");
    } else if buffer.starts_with(".#") {
        offset -= 2;
        prev_buff.push_str("..");
    } else if buffer.starts_with("..#") {
        offset -= 1;
        prev_buff.push_str(".");
    }

    prev_buff.push_str(buffer);

    if !prev_buff.ends_with("....") {
        prev_buff.push_str("....");
    }

    let mut new_buffer = String::new();
    for i in 0..prev_buff.len() - 4 {
        match &prev_buff[i..i+5] {
            "##.##" => new_buffer.push_str("#"),
            "....#" => new_buffer.push_str("."),
            ".#.#." => new_buffer.push_str("#"),
            "..###" => new_buffer.push_str("."),
            "##..." => new_buffer.push_str("#"),
            "#####" => new_buffer.push_str("."),
            "###.#" => new_buffer.push_str("#"),
            ".##.." => new_buffer.push_str("."),
            "..##." => new_buffer.push_str("."),
            "...##" => new_buffer.push_str("#"),
            "####." => new_buffer.push_str("."),
            "###.." => new_buffer.push_str("."),
            ".####" => new_buffer.push_str("#"),
            "#...#" => new_buffer.push_str("#"),
            "....." => new_buffer.push_str("."),
            "..#.." => new_buffer.push_str("."),
            "#..##" => new_buffer.push_str("."),
            "#.#.#" => new_buffer.push_str("#"),
            ".#.##" => new_buffer.push_str("#"),
            ".###." => new_buffer.push_str("."),
            "##..#" => new_buffer.push_str("."),
            ".#..." => new_buffer.push_str("#"),
            ".#..#" => new_buffer.push_str("#"),
            "...#." => new_buffer.push_str("."),
            "#.#.." => new_buffer.push_str("."),
            "#...." => new_buffer.push_str("."),
            "##.#." => new_buffer.push_str("."),
            "#.###" => new_buffer.push_str("."),
            ".##.#" => new_buffer.push_str("."),
            "#..#." => new_buffer.push_str("#"),
            "..#.#" => new_buffer.push_str("."),
            "#.##." => new_buffer.push_str("#"),
            _ => panic!("sdfsdf"),
        }
    }
    (offset + 2, new_buffer)
}
