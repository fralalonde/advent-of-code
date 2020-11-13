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
use std::rc::Rc;

const SERIAL: usize = 3463;

fn main() -> io::Result<()> {

    let max_power_cell: (usize, usize, isize) =
        (1..=298).flat_map(|y|
            (1..=298).map(move |x| (x, y, lvl_size(x, y, 3))
        )
    ).max_by_key(|cell| cell.2).unwrap();

    println!("max power cell ({},{}) level ({})", max_power_cell.0, max_power_cell.1, max_power_cell.2);

    let cache: Rc<RefCell<HashMap<(usize, usize, usize), isize>>> = Rc::new(RefCell::new(HashMap::new()));

    // fast way: lvl_size recursive + memoized
    // could have used the "cached" crate https://github.com/jaemk/cached
    let fast_power_grid: (usize, usize, usize, isize) =
        (1..=298).flat_map(|y| {
            let cache = cache.clone();
            (1..=298).flat_map(move |x| {
                let cache = cache.clone();
                (1..300 - x).map(move |z| (x, y, z, rlvl_size(x, y, z, cache.clone())))
            })
        })
        .max_by_key(|cell| cell.3).unwrap();

    println!("\nmax power grid (fast) ({},{},{}) level ({})", fast_power_grid.0, fast_power_grid.1, fast_power_grid.2, fast_power_grid.3);

    // brute force also works ok if ran with --release, ne
    let max_power_grid: (usize, usize, usize, isize) =
        (1..=298).flat_map(|y| {
            println!("line {}", y);
            (1..=298).flat_map(move |x|
                (1..300 - x).map(move |z| (x, y, z, lvl_size(x, y, z))))
        })
        .max_by_key(|cell| cell.3).unwrap();

    println!("\nmax power grid ({},{},{}) level ({})", max_power_grid.0, max_power_grid.1, max_power_grid.2, max_power_grid.3);

    Ok(())
}

fn lvl_size(x: usize, y: usize, z: usize) -> isize {
    (y..y+z).flat_map(|yy| (x..x+z).map(move |xx| lvl(xx, yy))).sum::<isize>()
}

fn rlvl_size(x: usize, y: usize, z: usize, cache: Rc<RefCell<HashMap<(usize, usize, usize), isize>>>) -> isize {
    let w = cache.borrow().get(&(x, y, z)).cloned();
    if let Some(w) = w {
        return w;
    } else {
        let sum = if z == 1 {
            lvl(x, y)
        } else {
            let row_sum = (x..x + z).map(|xx| lvl(xx, y)).sum::<isize>();
            let col_sum = (y + 1..y + z).map(|yy| lvl(x, yy)).sum::<isize>();
            let sub_sum = rlvl_size(x + 1, y + 1, z - 1, cache.clone());
            row_sum + col_sum + sub_sum
        };
        cache.borrow_mut().insert((x, y, z), sum);
        sum
    }
}

fn lvl(x: usize, y: usize) -> isize {
    let rack = x + 10;
    let lvl = rack * y;
    let salt = SERIAL + lvl;
    let rack_mult = rack * salt;
    let hund = (rack_mult % 1000) / 100;
    hund as isize - 5
}



