use std::io;
use std::io::BufRead;
use std::collections::{HashMap, HashSet};
use std::cmp::Ordering::*;

type Coord = (usize, usize);

#[derive(Clone, Copy, Debug)]
enum Corner {
    TopLeft, TopRight, BottomRight, BottomLeft,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Right, Down, Left, Up
}

#[derive(Clone, Debug)]
struct Cart {
    direction: Direction,
    cycle: usize,
}

use self::Direction::*;
use self::Corner::*;

fn main() -> io::Result<()> {

    let mut corners: HashMap<Coord, Corner> = HashMap::new();
    let mut intersections: HashSet<Coord> = HashSet::new();
    let mut carts: HashMap<Coord, Cart> = HashMap::new();

    let stdin = io::stdin();
    let mut y = 0;
    let mut on_track = false;
    for line in stdin.lock().lines() {
        for (x, b) in line?.as_bytes().iter().enumerate() {
            match *b as char {
                '/' => {
                    corners.insert((x, y), if !on_track { TopLeft } else { BottomRight });
                    on_track = !on_track;
                },
                '\\' => {
                    corners.insert((x, y), if !on_track { BottomLeft } else { TopRight });
                    on_track = !on_track;
                },
                '+' => {intersections.insert((x,y));},
                '>' => {carts.insert((x,y), Cart { direction: Right, cycle: 0});},
                '<' => {carts.insert((x,y), Cart { direction: Left, cycle: 0});},
                '^' => {carts.insert((x,y), Cart { direction: Up, cycle: 0});},
                'v' => {carts.insert((x,y), Cart { direction: Down, cycle: 0});},
                _ => {}
            }
        }
        y += 1;
    }

    loop {
        let mut sorted_coords: Vec<Coord> = carts.keys().cloned().collect();
        sorted_coords.sort_by(|a, b| match a.1.cmp(&b.1) {
            Equal => a.0.cmp(&b.0),
            z => z,
        });
        for pos in &sorted_coords {
            if let Some(mut cart) = carts.remove(pos) {
                let new_pos = match cart.direction {
                    Right => (pos.0 + 1, pos.1),
                    Left => (pos.0 - 1, pos.1),
                    Down => (pos.0, pos.1 + 1),
                    Up => (pos.0, pos.1 - 1),
                };
                if let Some(_other) = carts.remove(&new_pos) {
                    println!("collision at {:?}", new_pos);
                    continue;
                }

                if let Some(_xsec) = intersections.get(&new_pos) {
                    let new_dir_idx = cart.cycle % 3;
                    cart.direction = match cart.direction {
                        Left => [Down, Left, Up][new_dir_idx],
                        Right => [Up, Right, Down][new_dir_idx],
                        Up => [Left, Up, Right][new_dir_idx],
                        Down => [Right, Down, Left][new_dir_idx],
                    };
                    cart.cycle += 1;
                } else if let Some(corner) = corners.get(&new_pos) {
                    cart.direction = match (corner, cart.direction) {
                        (TopRight, Right) => Down,
                        (TopRight, Up) => Left,
                        (TopLeft, Up) => Right,
                        (TopLeft, Left) => Down,
                        (BottomRight, Right) => Up,
                        (BottomRight, Down) => Left,
                        (BottomLeft, Down) => Right,
                        (BottomLeft, Left) => Up,
                        _ => panic!("CART OFF THE RAIL")
                    }
                }
                carts.insert(new_pos, cart);
            }
        }

        if carts.len() == 1 {
            let last_cart = carts.iter().next().unwrap();
            println!("last cart {:?}", last_cart);
            break
        }
    }
    Ok(())
}
