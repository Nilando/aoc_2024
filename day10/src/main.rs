use std::collections::HashSet;

fn find_paths(map: &Vec<Vec<u8>>, (x, y): (usize, usize), finishes: &mut HashSet<(usize, usize)>) -> usize {
    let current_height = map[y][x];

    if current_height == 9 {
        finishes.insert((x, y));
        return 1;
    }

    let mut paths = 0;

    if x > 0 {
        let next = map[y][x - 1];
        if next == current_height + 1 {
            paths += find_paths(map, (x - 1, y), finishes);
        }
    }
    if x < map[y].len() - 1 {
        let next = map[y][x + 1];
        if next == current_height + 1 {
            paths += find_paths(map, (x + 1, y), finishes);
        }
    }
    if y > 0 {
        let next = map[y - 1][x];
        if next == current_height + 1 {
            paths += find_paths(map, (x, y - 1), finishes);
        }
    }
    if y < map.len() - 1 {
        let next = map[y + 1][x];
        if next == current_height + 1 {
            paths += find_paths(map, (x, y + 1), finishes);
        }
    }

    return paths
}


fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let mut map: Vec<Vec<u8>> = vec![];
    let mut part1 = 0;
    let mut part2 = 0;

    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c as u8 - 48);
        }

        map.push(row);
    }

    for (y, row) in map.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            if *height == 0 {
                let mut paths = HashSet::new();
                part2 += find_paths(&map, (x, y), &mut paths);
                part1 += paths.len();
            }
        }
    }

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
