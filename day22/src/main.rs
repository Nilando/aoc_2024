use std::collections::{HashMap, VecDeque};

fn main() {
    let input = std::include_str!("../input.txt");
    let mut part1 = 0;
    let mut cached_totals = HashMap::<(isize, isize, isize, isize), isize>::new();
    let mut part2 = 0;

    for line in input.lines() {
        let mut secret_number: usize = line.trim().parse().unwrap();
        let mut cached_price_changes = HashMap::new();
        let mut prev_changes = VecDeque::new();

        for i in 0..2000 {
            //println!("================== {i} ==================");
            let old_price: isize = (secret_number % 10) as isize;
            secret_number = next_number(secret_number);
            let new_price: isize = (secret_number % 10) as isize;
            let change: isize = new_price - old_price;

            if prev_changes.len() == 4 {
                prev_changes.pop_front();
                prev_changes.push_back(change);
                let change_sequence = (prev_changes[0], prev_changes[1], prev_changes[2], prev_changes[3]);
                if cached_price_changes.get(&change_sequence).is_some() {
                    continue;
                } else {
                    //println!("sequence: {:?}; price: {}", change_sequence, new_price);
                    cached_price_changes.insert(change_sequence, new_price);
                }
            } else {
                prev_changes.push_back(change);
            }
        }

        for (k, v) in cached_price_changes.into_iter() {
            if let Some(total) = cached_totals.get_mut(&k) {
                *total += v;

                if *total >= part2 {
                    println!("found new best: {:?}; {}", k, *total);
                    part2 = *total;
                }
            } else {
                cached_totals.insert(k, v);
                if v > part2 {
                    part2 = v;
                }
            }
        }

        //println!("{line}: {secret_number}");
        part1 += secret_number;
    }


    println!("part1: {part1}");
    println!("part2: {part2}");
}


fn next_number(secret_number: usize) -> usize {
    let r1 = prune((secret_number * 64) ^ secret_number);
    let r2 = prune((r1 / 32) ^ r1);
    
    prune((r2 * 2048) ^ r2)
}

fn prune(n: usize) -> usize {
    n % 16777216
}
