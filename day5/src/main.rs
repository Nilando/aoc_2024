use std::collections::HashSet;

fn main() {
    let file = std::fs::read_to_string("./input.txt").unwrap();
    let mut part1_sum = 0;
    let mut part2_sum = 0;
    let mut updates_flag = false;
    let mut rules: HashSet<(usize, usize)> = HashSet::new();

    for line in file.lines() {
        if line.trim().is_empty() {
            updates_flag = true;
            continue;
        }

        if !updates_flag {
            let pages: Vec<&str> = line.split('|').collect();
            let before_page: usize = pages[0].parse().unwrap();
            let after_page: usize = pages[1].parse().unwrap();

            rules.insert((before_page, after_page));
        } else {
            let mut update_pages: Vec<usize> = line.split(',').map(|page| page.parse().unwrap()).collect();
            let mut bad_manual = false;

            'bad: for i in 0..update_pages.len() {
                let before_page = update_pages[i];

                for k in (i+1)..update_pages.len() {
                    let after_page = update_pages[k];
                    if rules.contains(&(after_page, before_page)) {
                        bad_manual = true;
                        break 'bad;
                    }
                }
            }

            if bad_manual {
                update_pages.sort_by(|a, b| {
                    if rules.contains(&(*a, *b)) {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Less
                    }
                });
            }

            let idx = update_pages.len() / 2;
            if !bad_manual {
                part1_sum += update_pages[idx];
            } else {
                part2_sum += update_pages[idx];
            }
        }
    }

    println!("part1: {}", part1_sum);
    println!("part2: {}", part2_sum);
}
