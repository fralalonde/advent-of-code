extern crate failure;

struct Node {
    name: String,
    weight: u32,
    true_weight: u32,
    child_names: Vec<String>
}

impl Node {
    fn true_weight(&self, nodes: &HashMap<String, Node>) -> Vec<u32> {
        let sub: Vec<&Node> = self.child_names.iter().map(|c| nodes.get(c).unwrap()).collect();

        let weights: Vec<Vec<u32>> = sub.iter().map(|n| n.true_weight(nodes)).collect();

        // check
        let mut last_weight: Vec<u32> = vec![];
        let mut weight_sum = 0;
        for weight in weights {
            let new_sum: u32 = weight.iter().sum();
            if !last_weight.is_empty() {
                let old_sum: u32 = last_weight.iter().sum();
                if last_weight.iter().map(|x| *x as u32).sum::<u32>() != new_sum {
                    panic!("Weight of {:?} ({}) does not match {:?} ({})", last_weight, old_sum, weight, new_sum)
                }
            }
            last_weight = weight;
            weight_sum += new_sum;
        }

        vec![self.weight, weight_sum]
    }

//    fn unbal(&self, nodes: &HashMap<String, Node>) -> Option<(String, Vec<u32>)> {
//        // optimize: could check that child size > 2
//
//        let weights: Vec<u32> = sub.iter().map(|s| s.weight).collect();
//        if  weights.iter().next().unwrap() * weights.len() as u32 != weights.iter().map(|x| *x).sum() {
//            Some((self.name.clone(), weights))
//        } else {
//            for c in sub {
//                if let Some(u) = c.unbal(nodes) {
//                    return Some(u);
//                }
//            }
//            None
//        }
//    }
}

use std::collections::HashMap;

use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    run().unwrap();
}

fn run() -> Result<(), failure::Error> {
    let mut index: HashMap<String, String> = HashMap::new();
    let mut nodes: HashMap<String, Node> = HashMap::new();

    let f = File::open("data")?;
    let file = BufReader::new(&f);
    let mut lines = file.lines().into_iter();

    while let Some(Ok(line)) = lines.next() {
        let name = line;
        println!("name {}", name);

        let weight = lines.next().unwrap()?.parse()?;
        let mut child_names = Vec::new();

        while let Some(Ok(child)) = lines.next() {
            if !child.is_empty() {
                index.insert(child.clone(), name.clone());
                child_names.push(child.clone());
            } else {
                break;
            }
        }

        nodes.insert(name.clone(), Node {
            name,
            weight,
            true_weight: 1,
            child_names
        });
    }

    let mut parent: &String = &nodes.values().next().unwrap().name;
    while let Some(p) = index.get(parent) {
        parent = p;
    }

    println!("parent {}", parent);

    let parent_node: &Node = nodes.get(parent).unwrap();
    println!("parent true weight {:?}", parent_node.true_weight(&nodes));

//    println!("unbal {:?}", nodes.get(parent).unwrap().unbal(&nodes));

    Ok(())
}


//#[cfg(test)]
//pub fn test1() {
//    assert_eq!(5, count(vec![0,2,7,0]));
//}