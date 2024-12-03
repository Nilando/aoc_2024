fn solution() {
    let file = std::fs::read_to_string("./input.txt").unwrap();
    let mut chars = file.chars().peekable();
    let mut part1: usize = 0;
    let mut part2: usize = 0;
    let mut flag = true;

    loop {
        if chars.peek() == None {
            break;
        }

        if chars.peek() == Some(&'d') {
            chars.next();

            if chars.peek() != Some(&'o') {
                continue
            }
            chars.next();

            if chars.peek() == Some(&'(') {
                chars.next();
                if chars.peek() == Some(&')') {
                    flag = true;
                    chars.next();
                    continue
                }
            }

            if chars.peek() != Some(&'n') {
                continue
            }
            chars.next();

            if chars.peek() != Some(&'\'') {
                continue
            }
            chars.next();

            if chars.peek() != Some(&'t') {
                continue
            }
            chars.next();

            if chars.peek() != Some(&'(') {
                continue
            }
            chars.next();

            if chars.peek() != Some(&')') {
                continue
            }
            chars.next();

            flag = false;
            continue;
        }

        if chars.next() != Some('m') {
            continue
        }

        if chars.peek() != Some(&'u') {
            continue;
        }
        chars.next();

        if chars.peek() != Some(&'l') {
            continue;
        }
        chars.next();

        if chars.peek() != Some(&'(') {
            continue;
        }
        chars.next();

        let mut x = String::new();
        while let Some(p) = chars.peek() {
            if p.is_digit(10) {
                let c = chars.next().unwrap();
                x.push(c);
            } else {
                break;
            }
        }

        if chars.peek() != Some(&',') {
            continue;
        }
        chars.next();

        let mut y = String::new();
        while let Some(p) = chars.peek() {
            if p.is_digit(10) {
                let c = chars.next().unwrap();
                y.push(c);
            } else {
                break;
            }
        }

        if chars.peek() != Some(&')') {
            continue;
        }
        chars.next();

        part1 += x.parse::<usize>().unwrap() * y.parse::<usize>().unwrap();
        if flag {
            part2 += x.parse::<usize>().unwrap() * y.parse::<usize>().unwrap();
        }
    }

    println!("part1: {part1}");
    println!("part2: {part2}");
}

fn main() {
    solution()
}
