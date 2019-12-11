use anyhow::*;
use linked_hash_set::*;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default, Clone)]
struct Object {
    parent: Option<String>,
    children: Vec<String>,
}

fn count(objects: &HashMap<String, Object>, orig: &Option<String>, c: usize) -> usize {
    if let Some(pid) = orig {
        count(objects, &objects.get(pid).unwrap().parent, c + 1)
    } else {
        c
    }
}

fn to_root(
    objects: &HashMap<String, Object>,
    orig: &Option<String>,
    mut c: Vec<String>,
) -> Vec<String> {
    if let Some(pid) = orig {
        c.push(pid.to_string());
        to_root(objects, &objects.get(pid.as_str()).unwrap().parent, c)
    } else {
        c
    }
}

fn to_parent(
    objects: &HashMap<String, Object>,
    pname: &String,
    orig: &Option<String>,
    c: usize,
) -> usize {
    if let Some(pid) = orig {
        if pname.eq(pid) {
            return c;
        }
        to_parent(objects, pname, &objects.get(pid).unwrap().parent, c + 1)
    } else {
        panic!("node not found")
    }
}

fn main() -> Result<()> {
    let f = File::open("06/input")?;
    let file = BufReader::new(&f);
    let mut objects = HashMap::new();
    file.lines()
        .into_iter()
        .filter_map(|x| x.ok())
        .for_each(|line| {
            let ab: Vec<&str> = line.split(")").collect();
            objects
                .entry(ab[0].to_string())
                .or_insert(Object::default())
                .children
                .push(ab[1].to_string());
            objects
                .entry(ab[1].to_string())
                .or_insert(Object::default())
                .parent = Some(ab[0].to_string());
        });

    println!(
        "{}",
        objects
            .values()
            .map(|o| count(&objects, &o.parent, 0))
            .sum::<usize>()
    );
    let san_path: LinkedHashSet<String> = objects
        .values()
        .flat_map(|_o| to_root(&objects, &Some("SAN".to_string()), vec![]))
        .collect();

    let cross = objects
        .values()
        .flat_map(|o| to_root(&objects, &Some("YOU".to_string()), vec![]))
        .find(|x| san_path.contains(x))
        .unwrap();

    let a = to_parent(&objects, &cross, &Some("SAN".to_string()), 0);
    let b = to_parent(&objects, &cross, &Some("YOU".to_string()), 0);

    println!("cross {} a {} b {} len {}", cross, a, b, a + b - 2);

    Ok(())
}

#[cfg(test)]
mod test {

    #[test]
    fn tests() {}
}
