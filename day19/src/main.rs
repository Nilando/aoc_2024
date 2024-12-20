use std::collections::HashMap;
use regex::Regex;

fn main() {
    let input = std::include_str!("../input.txt");
    let mut available_towels: Vec<&str> = vec![];
    let mut part1 = 0;
    let mut part2 = 0;

    for (i, line) in input.lines().enumerate() {
        //println!("i: {i}");
        if i == 0 {
            available_towels = line.split(", ").collect();

            //println!("{:?}", available_towels);
            /*
            available_towels = compact_towels(available_towels);
            println!("{:?}", available_towels);
            */
            continue;
        }

        if i == 1 {
            continue;
        }

        /*
        if arrange_towels(&available_towels, line) {
            part1 += 1;
            part2 += part2_arrange_towels(&available_towels, line);
        }
        */
        let x = part2_arrange_towels(&available_towels, line);
        if x != 0 {
            part1 += 1;
            part2 += x;
        }
    } 

    println!("part1 {}", part1);
    println!("part2 {}", part2);
}

fn compact_towels(towels: Vec<&str>) -> Vec<&str> {
    let mut towels_to_remove = vec![];

    for towel in towels.iter() {
        let towels_minus_one: Vec<&str> = towels.iter().filter(|t| *t != towel).map(|t| *t).collect();

        if arrange_towels(&towels_minus_one,  towel) {
            towels_to_remove.push(towel.clone());
        }
    }

    towels.into_iter().filter(|t| !towels_to_remove.contains(&t)).collect()
}

fn arrange_towels(patterns: &[&str], design: &str) -> bool {
    if design.is_empty() {
        return true;
    }

    for pattern in patterns.iter() {
        let pat_len = pattern.len();

        if design.len() < pat_len {
            continue;
        }

        if **pattern == design[0..pat_len] {
            if arrange_towels(patterns, &design[pat_len..design.len()]) {
                return true
            }
        }
    }

    return false
}

fn part2_arrange_towels(patterns: &[&str], design: &str) -> usize {
    let mut matches = HashMap::<usize, Vec<usize>>::new();
    //println!("design: {}", design);

    for pattern in patterns.iter() {
        let re: Regex = Regex::new(*pattern).unwrap();
        let mut locs = re.capture_locations();

        if re.captures_read(&mut locs, design).is_none() {
            continue;
        }
        let mut start = 0;

        while let Some(mat) = re.find(&design[start..]) {
            let match_start = start + mat.start();
            let match_end = start + mat.end();
            //println!("Found match: {}", &design[match_start..match_end]);
            start = match_start + 1; // Move one position forward for overlapping
            if let Some(ends) = matches.get_mut(&match_start) {
                ends.push(match_end);
            } else {
                matches.insert(match_start, vec![match_end]);
            }
        }
    }

    let mut paths_counter = vec![0; design.len() + 1];
    paths_counter[0] = 1;
    for i in 0..design.len() {
        if paths_counter[i] == 0 {
            continue;
        }

        if let Some(ends) = matches.get(&i) {
            for k in ends.iter() {
                paths_counter[*k] += paths_counter[i];
            }
        }
    }

    //println!("matches: {:?}", matches);

    return *paths_counter.last().unwrap();
}
