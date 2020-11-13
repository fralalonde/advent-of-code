pub fn run(lines: &mut [Vec<u8>]) {

    let mut two = Vec::new();
    let mut thr = Vec::new();

    for s in lines {
        let mut has2 = false;
        let mut has3 = false;
        s.sort();

        let mut prev = s[0];
        let mut count = 1;
        for c in &s[1..] {
            if *c == prev {
                count += 1
            } else {
                match count {
                    2 => has2 = true,
                    3 => has3 = true,
                    _ => {}
                }
                count = 1;
            }
            prev = *c;
        }

        match count {
            2 => has2 = true,
            3 => has3 = true,
            _ => {}
        }

        if has2 { two.push(s.clone()); }
        if has3 { thr.push(s.clone()); }
    }

    println!("{} * {} = {}", two.len(), thr.len(), two.len() * thr.len())

}

