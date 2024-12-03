use std::fs::File;
use std::io::{BufReader, BufRead};

fn is_report_safe(mut levels: impl Iterator<Item = usize>) -> bool {
    let mut prev: usize = levels.next().unwrap();
    let mut curr: usize = levels.next().unwrap();
    let increasing: bool = prev < curr;

    if prev.abs_diff(curr) > 3 
    || prev == curr {
        return false;
    }

    for level in levels {
        prev = curr;
        curr = level;

        if prev.abs_diff(curr) > 3 
        || prev == curr 
        || (prev < curr && !increasing)
        || (prev > curr && increasing) {
            return false;
        }
    }

    return true;
}

fn part1() {
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut safe_counter = 0;

    for line in reader.lines().map(|line| line.unwrap()) {
        let levels = line.split_whitespace().map(|lvl| lvl.parse::<usize>().unwrap());
        if is_report_safe(levels) {
            safe_counter += 1;
        }
    }

    println!("safe counter(no dampener): {}", safe_counter);
}

fn part2() {
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut safe_counter = 0;

    for line in reader.lines().map(|line| line.unwrap()) {
        let levels = line.split_whitespace().map(|lvl| lvl.parse::<usize>().unwrap());
        let len = levels.clone().count();

        for i in 0..len {
            let patched_levels = levels
                .clone()
                .enumerate()
                .filter(|(idx, _)| *idx != i)
                .map(|(_, lvl)| lvl);

            if is_report_safe(patched_levels) {
                safe_counter += 1;
                break;
            }
        }
    }

    println!("safe counter(with dampener): {}", safe_counter);
}

fn main() {
    part1();
    part2();
}
