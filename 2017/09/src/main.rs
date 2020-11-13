extern crate failure;
use std::collections::HashMap;

use std::io::Read;
use std::fs::File;

#[derive(Debug, Clone)]
pub enum Node {
    Group(Vec<Node>),
    Garbage(Vec<String>),
}

fn main() {
    run().unwrap();
}

fn run() -> Result<(), failure::Error> {

    let mut group = 0;
    let mut score = 0;
    let mut garbage = false;
    let mut gcount = 0;

    let mut s = String::new();
    File::open("data").and_then(|mut f| f.read_to_string(&mut s))?;

    let mut i = s.chars();

    while let Some(ch) = i.next() {
        if garbage {
            match ch {
                '!' => { i.next(); },
                '>' => garbage = false,
                cha => gcount += 1,
            }
        } else {
            match ch {
                '{' => group += 1,
                '}' => group -= 1,
                '<' => garbage = true,
//                ',' => {},
                ch => {}
            }
        }
//        match ch {
//            '!' => { i.next(); },
//            '{' => if !garbage {group += 1; score += group } else { gcount += 1 },
//            '}' => if !garbage {group -= 1; } else { gcount += 1 },
//            '<' => if !garbage { garbage = true } else { gcount += 1 },
//            '>' => garbage = false,
//            cha => gcount += 1,
//        }
    }

    println!("biggest {:?}", score);
    println!("gcount {:?}", gcount);

    Ok(())

}