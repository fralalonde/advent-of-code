extern crate regex;

use std::fs::File;
use std::io::{BufReader, self, Read};

const FILENAME: &'static str = "input";
//const FILENAME: &'static str = "test";

struct Node {
    nodes: Vec<Node>,
    meta:  Vec<usize>,
}

fn main() -> io::Result<()> {
    let f = File::open(FILENAME)?;
    let mut file = BufReader::new(&f);
    let mut s = String::new();
    file.read_to_string(&mut s)?;

    let nums: Vec<usize> = s.split_whitespace().into_iter().map(|s| s.parse::<usize>().unwrap()).collect();

    let mut i = nums.into_iter();
    let root = read_node(&mut i);

    println!("meta {}", meta_sum(&root));
    println!("value {}", node_val(&root));

    Ok(())
}

fn meta_sum(node: &Node) -> usize {
    node.nodes.iter().map(|n| meta_sum(n)).sum::<usize>() +
        node.meta.iter().sum::<usize>()
}

fn node_val(node: &Node) -> usize {
    if node.nodes.is_empty() {
        node.meta.iter().sum::<usize>()
    } else {
        node.meta.iter().filter_map(|idx| node.nodes.get(*idx - 1).map(|n| node_val(n))).sum()
    }
}

fn read_node(i: &mut Iterator<Item=usize>) -> Node {
    let ncount = i.next().unwrap();
    let mcount = i.next().unwrap();
    let mut nodes = vec![];
    for _ in 0..ncount {
        nodes.push(read_node(i));
    }
    Node {
        nodes,
        meta: i.take(mcount).collect(),
    }
}




