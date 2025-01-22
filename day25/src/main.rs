fn main() {
    //let input = std::include_str!("../test.txt");
    let input = std::include_str!("../input.txt");

    let mut locks = vec![];
    let mut keys = vec![];
    let mut reading_lock = true;
    let mut current_heights = [0; 5];
    let mut lines_read = 0;

    for line in input.lines() {
        lines_read += 1;

        if line.trim().is_empty() {
            lines_read = 0;
            continue;
        }

        if lines_read == 1 {
            current_heights = [0; 5];

            if line.contains('#') {
                reading_lock = true;
            } else {
                reading_lock = false;
            }

            continue;
        }

        if lines_read == 7 {
            if reading_lock {
                locks.push(current_heights);
            } else {
                keys.push(current_heights);
            }

            continue;
        }


        for (i, c) in line.trim().chars().enumerate() {
            if c == '#' {
                current_heights[i] += 1;
            }
        }
    }


    let mut part1 = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            let mut fits = true;
            for i in 0..5 {
                if lock[i] + key[i] > 5 {
                    fits = false;
                    break;
                }
            }
            if fits {
                part1 += 1;
            }
        }
    }

    //println!("locks: {:#?}", locks);
    //println!("keys: {:#?}", keys);
    println!("part1: {part1}");
}
