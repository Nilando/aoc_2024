use std::fs::File;
use std::io::{BufReader, BufRead};

fn is_report_safe(levels: &Vec<usize>) -> bool {
    let mut iter = levels.iter();
    let mut prev: usize = *iter.next().unwrap();
    let mut curr: usize = *iter.next().unwrap();
    let increasing: bool = prev < curr;

    if prev.abs_diff(curr) > 3 
    || prev == curr {
        return false;
    }

    for level in iter {
        prev = curr;
        curr = *level;

        if prev.abs_diff(curr) > 3 
        || prev == curr {
            return false;
        }

        if prev < curr {
            if !increasing {
                return false;
            }
        } else {
            if increasing {
                return false;
            }
        }

    }

    return true;
}

fn part1() {
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut safe_counter = 0;

    for line in reader.lines().map(|line| line.unwrap()) {
        let levels: Vec<usize> = line.split_whitespace().map(|lvl| lvl.parse::<usize>().unwrap()).collect();
        if is_report_safe(&levels) {
            safe_counter += 1;
        }
    }

    println!("safe counter: {}", safe_counter);
}

fn part2() {
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut safe_counter = 0;

    for line in reader.lines().map(|line| line.unwrap()) {
        let levels: Vec<usize> = line.split_whitespace().map(|lvl| lvl.parse::<usize>().unwrap()).collect();
        if is_report_safe(&levels) {
            safe_counter += 1;
        } else {
            for i in 0..levels.len() {
                let mut patched_levels = levels.clone();

                patched_levels.remove(i);

                if is_report_safe(&patched_levels) {
                    safe_counter += 1;
                    break;
                }
            }
        }
    }

    println!("safe counter(with problem dampener): {}", safe_counter);
}

fn main() {
    part1();
    part2();
}
