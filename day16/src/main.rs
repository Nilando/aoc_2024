use std::collections::HashSet;
use colored::Colorize;

fn main() {
    let input = std::include_str!("../input.txt");
    //let input = std::include_str!("../test.txt");
    //let input = std::include_str!("../testb.txt");
    //let input = std::include_str!("../testc.txt");
    let mut map: Vec<Vec<usize>> = vec![];
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (row, line) in input.lines().enumerate() {
        let mut r = vec![];

        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                r.push(usize::MAX);
            } else if c == '.' {
                r.push(0);
            } else if c == 'S' {
                r.push(1);
                start = (row, col);
            } else {
                end = (row, col);
                r.push(0);
            }

        }

        map.push(r);
    }


    let r = search(&mut map, start, end, 1, (0, 1)).unwrap();

    println!("part1 {}", r.0 - 1);
    println!("part2 {}", r.1.len());


    for (row, line) in map.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if *c == usize::MAX {
                //print!("{:>5}", '#');
                print!("{}", '#');
            } else if r.1.get(&(row, col)).is_some() {
                //print!("{:>5}", 'O');
                print!("{}", "0".red());
            } else {
                //print!("{:>5}", c);
                print!("{}", '.');
            }
        }
        println!("");
    }
}

    fn search(map: &mut Vec<Vec<usize>>, pos: (usize, usize), end: (usize, usize), cost: usize, prev_dir: (isize, isize)) -> Option<(usize, HashSet<(usize, usize)>)> {
    if end == pos {
        return Some((cost, HashSet::from([end])));
    }

    let mut best_path = None;
    let mut pivot_flag = false;
    let mut wall_count = 0;

    for (r, c) in [(1, 0), (-1, 0), (0, 1), (0, -1)].into_iter() {
        let new_pos = ((pos.0 as isize + r) as usize, (pos.1 as isize + c) as usize);
        let p = &mut map[new_pos.0 as usize][new_pos.1 as usize];

        if *p == usize::MAX {
            wall_count += 1;
            continue;
        }

        let mut new_cost = cost + 1;

        if (r, c) != prev_dir {
            new_cost += 1000;
        }

        if *p != 0 {
            if *p < new_cost {
                continue;
            }
        }

        *p = new_cost;

        if let Some((found_cost, mut found_vt)) = search(map, new_pos, end, new_cost, (r, c)) {
            found_vt.insert(pos);
            if let Some((best_cost, ref mut best_vt)) = best_path {
                if found_cost < best_cost {
                    pivot_flag = (r, c) != prev_dir;
                    best_path = Some((found_cost, found_vt));
                } else if found_cost == best_cost {
                    best_vt.extend(found_vt.iter());
                }
            } else {
                pivot_flag = (r, c) != prev_dir;
                best_path = Some((found_cost, found_vt));
            }
        }
    }


    if pivot_flag {
        map[pos.0][pos.1] += 1000;
    }

    if wall_count == 3 {
        map[pos.0][pos.1] = usize::MAX;
    }

    best_path
}
