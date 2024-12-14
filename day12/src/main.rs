use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    //let input = std::fs::read_to_string("./test.txt").unwrap();
    let mut map: Vec<Vec<char>> = vec![];

    for line in input.lines() {
        let mut row = vec![];

        for char in line.chars() {
            row.push(char);
        }

        map.push(row);
    }

    let mut part1 = 0;
    let mut part2 = 0;

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] as u8 > 90 {
                continue;
            }

            let mut perimeter = 0;
            let mut area = 0;
            let mut sides = 0;
            let mut fences = HashMap::<(usize, usize), (bool, bool, bool, bool)>::new();

            search_region(&mut map, &mut perimeter, &mut area, &mut sides, &mut fences, (col, row));

            part1 += area * perimeter;
            part2 += area * sides;
        }
    }

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

fn search_region(map: &mut Vec<Vec<char>>, perimeter: &mut usize, area: &mut usize, sides: &mut usize, fences: &mut HashMap<(usize, usize), (bool, bool, bool, bool)>, (x, y): (usize, usize)){
    let current = map[y][x];
    let replaced = (current as u8 + 32) as char;
    let mut top_side = false;
    let mut right_side = false;
    let mut bottom_side = false;
    let mut left_side = false;
    map[y][x] = replaced;

    *area += 1;

    if y == 0 || (map[y-1][x] != replaced && map[y-1][x] != current) {
        *perimeter += 1;
        top_side = true
    }
    if x + 1 == map[0].len() || (map[y][x+1] != replaced && map[y][x+1] != current) {
        *perimeter += 1;
        right_side = true
    }
    if y + 1 == map.len() || (map[y+1][x] != current && map[y+1][x] != replaced) {
        *perimeter += 1;
        bottom_side = true;
    }
    if x == 0 || (map[y][x-1] != replaced && map[y][x-1] != current) {
        *perimeter += 1;
        left_side = true
    }

    fences.insert((x,y), (top_side, right_side, bottom_side, left_side));

    if top_side {
        if (left_side || !fences.get(&(x-1, y)).is_some_and(|g| g.0))
        && (right_side || !fences.get(&(x+1, y)).is_some_and(|g| g.0)) {
            *sides += 1;
        } else if (!left_side && fences.get(&(x-1, y)).is_some_and(|g| g.0))
            && (!right_side && fences.get(&(x+1, y)).is_some_and(|g| g.0)) {
            *sides -= 1;
        } 
    }
    if bottom_side {
        if (left_side || !fences.get(&(x-1, y)).is_some_and(|g| g.2))
        && (right_side || !fences.get(&(x+1, y)).is_some_and(|g| g.2)) {
            *sides += 1;
        } else if (!left_side && fences.get(&(x-1, y)).is_some_and(|g| g.2))
            && (!right_side && fences.get(&(x+1, y)).is_some_and(|g| g.2)) {
            *sides -= 1;
        } 
    }
    if right_side {
        if (top_side || !fences.get(&(x, y-1)).is_some_and(|g| g.1))
            && (bottom_side || !fences.get(&(x, y+1)).is_some_and(|g| g.1)) {
            *sides += 1;
        } else if (!top_side && fences.get(&(x, y-1)).is_some_and(|g| g.1))
            && (!bottom_side && fences.get(&(x, y+1)).is_some_and(|g| g.1)) {
            *sides -= 1;
        } 
    }
    if left_side {
        if (top_side || !fences.get(&(x, y-1)).is_some_and(|g| g.3))
        && (bottom_side || !fences.get(&(x, y+1)).is_some_and(|g| g.3)) {
            *sides += 1;
        } else if (!top_side && fences.get(&(x, y-1)).is_some_and(|g| g.3))
            && (!bottom_side && fences.get(&(x, y+1)).is_some_and(|g| g.3)) {
            *sides -= 1;
        } 
    }

    if !top_side && map[y-1][x] == current {
        search_region(map, perimeter, area, sides, fences, (x, y-1));
    }
    if !right_side && map[y][x+1] == current {
        search_region(map, perimeter, area, sides, fences, (x+1, y));
    }
    if !bottom_side && map[y+1][x] == current {
        search_region(map, perimeter, area, sides, fences, (x, y+1));
    }
    if !left_side && map[y][x-1] == current {
        search_region(map, perimeter, area, sides, fences, (x-1, y));
    }
}
