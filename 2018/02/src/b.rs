pub fn run(lines: &[Vec<u8>]) {
    for s in lines {
        for s2 in lines {
            let mut diff = 0;
            for (i, c) in s.iter().enumerate() {
                if s2[i] != *c {
                    diff += 1
                }
                if diff > 1 {break}
            }
            if diff == 1 {
                println!("{}", String::from_utf8(s.clone()).unwrap());
                println!("{}", String::from_utf8(s2.clone()).unwrap());
                return
            }
        }
    }
}
