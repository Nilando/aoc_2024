use std::collections::HashSet;

const ROWS: usize = 71;
const COLS: usize = 71;

// calc the shortest path
// if the next byte falls on that path
// re calc the shortest path

fn main() {
    let input = std::include_str!("../input.txt");
    let mut map: [[usize; COLS]; ROWS] = [[0; COLS]; ROWS];
    let mut shortest_path = HashSet::new();

    for (i, line) in input.lines().enumerate() {
        let mut iter = line.split(',');
        let x: usize = iter.next().unwrap().parse().unwrap();
        let y: usize = iter.next().unwrap().parse().unwrap();

        map[y][x] = usize::MAX;

        if i != 1024 && !shortest_path.contains(&(y, x))  {
            continue;
        }

        let mut search_map = map.clone();
        if let Some(steps) = search(&mut search_map) {
            //print_map(&search_map);
            shortest_path = find_shortest_path(&search_map);
            if i == 1024 {
                println!("part1: {}", steps);
            }
        } else {
            println!("part2: {}, {}", x, y);
            break;
        }
    }
}

fn print_map(map: &[[usize; COLS]; ROWS]) {
    for row in map.iter() {
        for p in row.iter() {
            if *p == 0 {
                print!(".");
            } else if *p == usize::MAX {
                print!("#");
            } else {
                print!("{}, ", *p);
            }
        }
        println!("");
    }
}

// couldn't get this to work! would speed up part2 dramatically
fn find_shortest_path(map: &[[usize; COLS]; ROWS]) -> HashSet<(usize, usize)> {
    let mut pos = (ROWS - 1, COLS - 1);
    let mut visited_positions = HashSet::from([pos]);

    loop {
        //println!("pos: {:?}", pos);
        
        let steps = &map[pos.0 as usize][pos.1 as usize];
        if *steps == 0 {
            break;
        }

        for (r, c) in [(1, 0), (-1, 0), (0, 1), (0, -1)].into_iter() {
            if (pos.0 == 0 && r == -1) || (pos.1 == 0 && c == -1) || (pos.0 == ROWS - 1 && r == 1) || (pos.1 == COLS - 1 && c == 1) {
                continue;
            }

            let new_pos = ((pos.0 as isize + r) as usize, (pos.1 as isize + c) as usize);
            let p = &map[new_pos.0 as usize][new_pos.1 as usize];

            //println!("s: {}, p: {}", steps, *p);
            if *p == *steps - 1 {
                visited_positions.insert(new_pos);
                pos = new_pos;
                break;
            }
        }

        if pos == (0, 0) {
            break;
        }
    }

    visited_positions
}

fn search(map: &mut [[usize; COLS]; ROWS]) -> Option<usize> {
    let mut unvisited_nodes = Vec::from([((0, 0), 1)]);
    map[0][0] = 1;

    loop {
        if unvisited_nodes.is_empty() {
            return None
        }

        unvisited_nodes.sort_by(|a, b| b.1.cmp(&a.1));

        let (pos, steps) = unvisited_nodes.pop().unwrap();

        //print_map(&map);

        for (r, c) in [(1, 0), (-1, 0), (0, 1), (0, -1)].into_iter() {
            if (pos.0 == 0 && r == -1) || (pos.1 == 0 && c == -1) || (pos.0 == ROWS - 1 && r == 1) || (pos.1 == COLS - 1 && c == 1) {
                continue;
            }

            let new_pos = ((pos.0 as isize + r) as usize, (pos.1 as isize + c) as usize);
            let p = &mut map[new_pos.0 as usize][new_pos.1 as usize];

            if *p == usize::MAX {
                continue;
            }

            if new_pos == (ROWS - 1, COLS - 1) {
                *p = steps + 1;
                //println!("part1: {}", steps);
                return Some(steps);
            }

            if *p == 0 || *p > steps + 1 {
                //println!("insert unvisited: {}, {:?}", steps + 1, new_pos);
                *p = steps + 1;
                unvisited_nodes.push((new_pos, steps + 1));
            }
        }
    }
}
