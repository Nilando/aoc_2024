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

        if chars.peek() != Some(&'d') && chars.peek() != Some(&'m') {
            chars.next();
            continue;
        }

        if chars.next_if_eq(&'d').is_some()
        && chars.next_if_eq(&'o').is_some()
        {
            if chars.next_if_eq(&'(').is_some()
            && chars.next_if_eq(&')').is_some() {
                flag = true;
                continue;
            }

            if chars.next_if_eq(&'n').is_some()
            && chars.next_if_eq(&'\'').is_some() 
            && chars.next_if_eq(&'t').is_some()
            && chars.next_if_eq(&'(').is_some()
            && chars.next_if_eq(&')').is_some() {
                flag = false;
                continue;
            }
        }

        if chars.next_if_eq(&'m').is_some()
        && chars.next_if_eq(&'u').is_some() 
        && chars.next_if_eq(&'l').is_some()
        && chars.next_if_eq(&'(').is_some() {
            let mut x = String::new();
            while let Some(p) = chars.peek() {
                if p.is_digit(10) {
                    let c = chars.next().unwrap();
                    x.push(c);
                } else {
                    break;
                }
            }

            if chars.next_if_eq(&',').is_none() {
                continue;
            }

            let mut y = String::new();
            while let Some(p) = chars.peek() {
                if p.is_digit(10) {
                    let c = chars.next().unwrap();
                    y.push(c);
                } else {
                    break;
                }
            }

            if chars.next_if_eq(&')').is_none() {
                continue;
            }

            part1 += x.parse::<usize>().unwrap() * y.parse::<usize>().unwrap();
            if flag {
                part2 += x.parse::<usize>().unwrap() * y.parse::<usize>().unwrap();
            }
        }
    }

    println!("part1: {part1}");
    println!("part2: {part2}");
}

fn main() {
    solution()
}
