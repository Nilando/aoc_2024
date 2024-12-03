mod input;
use input::INPUT;
use std::collections::HashMap;

fn part1() {
    let mut list1 = vec![];
    let mut list2 = vec![];

    for (a, b) in INPUT.iter() {
        list1.push(a);
        list2.push(b);
    }

    list1.sort();
    list2.sort();


    let mut summed_distance = 0;

    for i in 0..1000 {
        let distance = list1[i].abs_diff(*list2[i]);

        summed_distance += distance;
    }

    println!("summed distance: {}", summed_distance);
}

fn part2() {
    let mut counts: HashMap<usize, usize> = HashMap::new();

    for (_, num) in INPUT.iter() {
        match counts.get(num) {
            None => {
                counts.insert(*num, 1);
            }
            Some(count) => {
                counts.insert(*num, count + 1);
            }
        }
    }

    let mut similarity_score = 0;

    for i in 0..1000 {
        let (num, _) = INPUT[i];

        match counts.get(&num) {
            None => {}
            Some(count) => {
                similarity_score += num * count;
            }
        }
    }

    println!("similarity score: {}", similarity_score);
}

fn main() {
    part1();
    part2();
}
