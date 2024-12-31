const WALL: usize = usize::MAX;

fn main() {
    let raw_map = std::include_str!("../input.txt");
    let (mut map, start, end) = parse_map(raw_map);
    let valid_positions = walk_race_track(&mut map, start, end);
    let max_steps = map[end.0][end.1];

    cheat_search(&map, &valid_positions, max_steps);
}

fn parse_map(raw_map: &str) -> (Vec<Vec<usize>>, (usize, usize), (usize, usize)) {
    let mut map: Vec<Vec<usize>> = vec![];
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (row, line) in raw_map.lines().enumerate() {
        let mut r = vec![];

        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                r.push(WALL);
            } else if c == '.' {
                r.push(0);
            } else if c == 'S' {
                r.push(0);
                start = (row, col);
            } else {
                end = (row, col);
                r.push(0);
            }

        }

        map.push(r);
    }

    (map, start, end)
}

fn walk_race_track(map: &mut Vec<Vec<usize>>, start: (usize, usize), end: (usize, usize)) -> Vec<(usize, usize)> {
    let mut pos = start;
    let mut visited_positions = vec![start];

    loop {
        let steps = map[pos.0][pos.1];

        for (r, c) in [(1, 0), (-1, 0), (0, 1), (0, -1)].into_iter() {
            let new_pos = ((pos.0 as isize + r) as usize, (pos.1 as isize + c) as usize);
            let p = &mut map[new_pos.0][new_pos.1];

            if *p == 0 && new_pos != start {
                visited_positions.push(new_pos);
                pos = new_pos;
                *p = steps + 1;
                break;
            }
        }

        if pos == end {
            break;
        }
    }

    visited_positions
}

fn print_map(map: &Vec<Vec<usize>>, visited: Option<&Vec<(usize, usize)>>) {
    for (row, ps) in map.iter().enumerate() {
        for (col, p) in ps.iter().enumerate() {
            if visited.is_some() && visited.unwrap().contains(&(row, col)) {
                print!("() ");
            } else if *p == WALL {
                print!("## ");
            } else {
                print!("{:0>2} ", map[row][col]);
            }
        }
        println!("");
    }
}

fn cheat_search(map: &Vec<Vec<usize>>, visited: &Vec<(usize, usize)>, max_steps: usize) {
    let mut part1 = 0;
    let mut part2 = 0;
    let cheat_len: isize = 20;
    let minimum_save = 100;

    for cheat_start in visited.iter() {
        let starting_steps = map[cheat_start.0][cheat_start.1];

        if starting_steps + minimum_save > max_steps {
            continue
        }

        for r in (cheat_len * -1)..=cheat_len {
            let d: isize = cheat_len as isize - r.abs();

            for c in (d * -1)..=d {
                let steps = (r.abs() + c.abs()) as usize;
                let new_pos = ((cheat_start.0 as isize + r), (cheat_start.1 as isize + c));

                if new_pos.0 < 0 || new_pos.1 < 0 || new_pos.0 as usize >= map.len() || new_pos.1 as usize >= map[0].len() {
                    continue;
                }

                let p = &map[new_pos.0 as usize][new_pos.1 as usize];

                if *p != WALL && *p > (starting_steps + steps) {
                    let savings = *p - (starting_steps + steps);

                    if savings >= minimum_save {
                        if steps == 2 {
                            part1 += 1;
                        } 
                        part2 += 1;
                    }
                }
            }
        }
    }

    println!("part1 {}", part1);
    println!("part2 {}", part2);
}
