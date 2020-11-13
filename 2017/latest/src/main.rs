#![feature(i128_type)]

extern crate failure;
use std::collections::HashMap;

use std::io::Read;
use std::fs::File;
use std::ascii::*;

fn main() {
    run().unwrap();
    run2().unwrap();
}

fn run() -> Result<(), failure::Error> {

    const LENS: [u8; 16] = [120,93,0,90,5,80,129,74,1,165,204,255,254,2,50,113];

    let mut data: [u8; 256] = [0; 256];
    let mut skip: usize = 0;
    let mut pos: usize = 0;

    for i in 0..data.len() {
        data[i] = i as u8
    }

    for i in 0..LENS.len() {
        if LENS[i] > 0 {
            let mut end = pos + (LENS[i] as i32 - 1) as usize;
            let mut beg = pos;
            while end > beg {
                // could XOR? fuck it just swap dumbly
                let z = data[end % 256];
                data[end % 256] = data[beg % 256];
                data[beg % 256] = z;
                end -= 1;
                beg += 1;
            }
        }
        pos += LENS[i] as usize + skip;
        skip += 1;
    }

    println!("first two mult {:?}", data[0] as u64 * data[1] as u64);

    Ok(())

}

fn run2() -> Result<(), failure::Error> {
    const SALT: [u8; 5] = [17, 31, 73, 47, 23];

    let mut lens = "120,93,0,90,5,80,129,74,1,165,204,255,254,2,50,113".to_string().into_bytes();
    lens.extend(SALT.iter());

    let mut data: [u8; 256] = [0; 256];
    let mut skip: usize = 0;
    let mut pos: usize = 0;

    for i in 0..data.len() {
        data[i] = i as u8
    }

    for _ in 0..64 {
        for i in 0..lens.len() {
            if lens[i] > 0 {
                let mut end = pos + (lens[i] as i32 - 1) as usize;
                let mut beg = pos;
                while end > beg {
                    // could XOR? fuck it just swap dumbly
                    let z = data[end % 256];
                    data[end % 256] = data[beg % 256];
                    data[beg % 256] = z;
                    end -= 1;
                    beg += 1;
                }
            }
            pos += lens[i] as usize + skip;
            skip += 1;
        }
    }

    println!("all {:?}", &data[..]);
    let mut dense: i128 = 0;
    for i in 0..16 {
        let mut z = data[i * 16];
        for j in 1..16 {
            z = z ^ data[(i * 16) + j];
        }
        dense = dense << 8;
        dense += z as i128;
        println!("hash: {:x}", dense);
    }

    Ok(())
}