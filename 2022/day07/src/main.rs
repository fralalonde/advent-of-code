use std::borrow::BorrowMut;
use std::cell::{RefCell};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::rc::Rc;

use linked_hash_map::LinkedHashMap;
use regex::Regex;

#[derive(Clone)]
pub enum Entry {
    Dir(Rc<RefCell<LinkedHashMap<String, Entry>>>),
    File(u32),
}

fn main() -> anyhow::Result<()> {
    let f = File::open("day07/input")?;
    let file = BufReader::new(&f);

    let root = build_dir(file)?;

    let mut smol = vec![];
    let root_size = tree_size(&root.as_ref().borrow(), &mut smol);
    println!("smol {}", smol.iter().sum::<u32>());

    let mut smol = vec![];
    tree_size2(&root.as_ref().borrow(), &mut smol);
    let free = 70000000 - root_size;
    let min_req = 30000000 - free;
    smol.sort();
    // smol.reverse();
    smol = smol.into_iter().filter_map(|s| (s > min_req).then(|| s)).collect();
    println!("req {min_req} smol {:?}", smol[0]);

    Ok(())
}

fn build_dir(file: impl BufRead) -> anyhow::Result<Rc<RefCell<LinkedHashMap<String, Entry>>>> {
    let lines = file.lines().filter_map(|l| l.ok()).into_iter();
    let root = Rc::new(RefCell::new(LinkedHashMap::new()));

    let mut path = Vec::new();
    let mut pwd: Rc<RefCell<LinkedHashMap<String, Entry>>> = root.clone();

    let re_cd = Regex::new(r"\$ cd (.*)")?;
    let re_dir = Regex::new(r"dir (.+)")?;
    let re_file = Regex::new(r"(\d+) (.+)")?;
    // ls ignored

    for line in lines {
        if let Some(caps) = re_cd.captures(&line) {
            match &caps[1] {
                "/" => {
                    path.clear();
                    pwd = root.clone();
                }
                ".." => {
                    pwd = path.pop().unwrap();
                }
                in_dir => {
                    let b = pwd.borrow().get(in_dir).cloned();
                    if let Some(Entry::Dir(new_dir)) = b {
                        path.push(pwd.clone());
                        *pwd.borrow_mut() = new_dir.clone();
                    } else { panic!("not a dir {}", in_dir) }
                }
            }
        } else if let Some(caps) = re_dir.captures(&line) {
            pwd.as_ref().borrow_mut().insert(caps[1].to_string(), Entry::Dir(Rc::new(RefCell::new(LinkedHashMap::new()))));
        } else if let Some(caps) = re_file.captures(&line) {
            pwd.as_ref().borrow_mut().insert(caps[2].to_string(), Entry::File(u32::from_str(&caps[1]).unwrap()));
        }
    }
    Ok(root)
}

fn dir_size(dir: &LinkedHashMap<String, Entry>) -> u32 {
    dir.iter().filter_map(|(_name, entry)| match entry {
        Entry::File(sz) => { Some(*sz) }
        _ => None
    }).sum::<u32>()
}

fn tree_size(dir: &LinkedHashMap<String, Entry>, smol: &mut Vec<u32>) -> u32 {
    let fs = dir_size(dir);
    let ss = dir.iter().filter_map(|(_name, entry)| match entry {
        Entry::Dir(d) => { Some(tree_size(&d.as_ref().borrow(), smol)) }
        _ => None
    }).sum::<u32>();
    if fs + ss <= 100000 {
        smol.push(fs + ss)
    }
    fs + ss
}

fn tree_size2(dir: &LinkedHashMap<String, Entry>, smol: &mut Vec<u32>) -> u32 {
    let fs = dir_size(dir);
    let ss = dir.iter().filter_map(|(_name, entry)| match entry {
        Entry::Dir(d) => { Some(tree_size2(&d.as_ref().borrow(), smol)) }
        _ => None
    }).sum::<u32>();

    smol.push(fs + ss);
    fs + ss
}

pub fn print_dir(dir: &LinkedHashMap<String, Entry>, depth: u32) {
    for e in dir.iter() {
        for _ in 0..depth {
            print!("  ");
        }
        match e.1 {
            Entry::Dir(d) => {
                println!("- {} (dir)", e.0);
                print_dir(&d.as_ref().borrow(), depth + 1)
            }
            Entry::File(sz) => { println!("- {} (file, size={})", e.0, sz); }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use std::io::BufReader;
    use crate::{build_dir, print_dir, dir_size, tree_size};

    static INPUT: &str = r"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn ting() -> anyhow::Result<()> {
        let root = build_dir(BufReader::new(INPUT.as_bytes()))?;
        print_dir(&root.as_ref().borrow(), 1);
        Ok(())
    }

    #[test]
    fn tong() -> anyhow::Result<()> {
        let root = build_dir(BufReader::new(INPUT.as_bytes()))?;
        let size = dir_size(&root.as_ref().borrow());
        assert_eq!(14848514 + 8504156, size/*cc.iter().sum::<u32>()*/);
        Ok(())
    }

    #[test]
    fn tang() -> anyhow::Result<()> {
        let root = build_dir(BufReader::new(INPUT.as_bytes()))?;
        let mut smol = vec![];
        let size = tree_size(&root.as_ref().borrow(), &mut smol);
        assert_eq!(48381165, size);
        assert_eq!(95437u32, smol.iter().sum());
        Ok(())
    }
}