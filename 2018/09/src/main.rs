extern crate regex;
extern crate linked_list;

use std::fs::File;
use std::io::{BufReader, self, Read};
use linked_list::{LinkedList, Cursor};

const FILENAME: &'static str = "input";
//const FILENAME: &'static str = "test";

//const PLAYERS: usize = 30;
//const LAST_MARBLE: usize = 5807;

const PLAYERS: usize = 447;
const LAST_MARBLE: usize = 7151000;

fn main() -> io::Result<()> {
    // rust arrays are a bitch, use vec.
    // FIXME allocate using mem::zeroed / set_length / Vec::from_raw_parts ?
    let mut scores: Vec<usize> = (0..PLAYERS).map(|_|0).collect();
    let mut marbles = linked_list::LinkedList::new();
    let mut cur = marbles.cursor();
    cur.insert(0);
    let mut idx = 0;
    // track len without marbles.len() because marbles is mutably borrowed by cursor... :(
    // TODO PR to have cursor expose underlying linkedlist.len()
    let mut len = 1;

    for new in 1..=LAST_MARBLE {
        if new % 23 == 0 {
            scores[(new - 1) % PLAYERS] += new;
            let new_idx = (idx + len - 7) % len;
            let offset: isize = new_idx - idx;
            let uffset = offset.abs() as usize;
            if offset < 0 {cur.seek_backward(uffset)} else {cur.seek_forward(uffset)};
            idx = new_idx;
            scores[(new - 1) % PLAYERS] += cur.remove().unwrap();
            len -= 1;
        } else {
            let new_idx = (idx + 2) % len;
            let offset: isize = new_idx - idx;
            let uffset = offset.abs() as usize;
            if offset < 0 {cur.seek_backward(uffset)} else {cur.seek_forward(uffset)};
            idx = new_idx;
            cur.insert(new);
            len += 1;
        }
        if new % 10000 ==0 {
            println!("marble {}", new);
        }
    }

    scores.sort();

    println!("high score {:?}", scores[scores.len() - 1]);

    Ok(())
}



