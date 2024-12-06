#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(PartialEq, Copy, Clone)]
enum Position {
    Visited(Direction),
    Obstructed,
    Unvisited,
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let mut map: Vec<Vec<Position>> = vec![];
    let mut guard_pos = (0, 0);
    let mut visit_count = 1;

    for (row, line) in input.lines().enumerate() {
        let mut positions = vec![];

        for (col, pos) in line.trim().chars().enumerate() {
            if pos == '#' {
                positions.push(Position::Obstructed);
            }

            if pos == '^' {
                guard_pos = (row, col);
                positions.push(Position::Visited(Direction::Up));
            }

            if pos == '.' {
                positions.push(Position::Unvisited);
            }
        }

        map.push(positions);
    }

    let original_guard_pos = guard_pos;
    let original_map = map.clone();
    println!("guard is at : {:?}", guard_pos);

    let mut direction = Direction::Up;
    let mut loops_created = 0;
    let mut visited_positions = vec![];

    loop {
        let (mut row, mut col) = guard_pos;

        match direction {
            Direction::Down => {
                if row == map.len() - 1 { break; }
                row += 1;
            }
            Direction::Left => {
                if col == 0 { break; }
                col -= 1;
            }
            Direction::Up => {
                if row == 0 { break; }
                row -= 1;
            }
            Direction::Right => {
                if col == map[0].len() - 1 { break; }
                col += 1;
            }
        }

        match map[row][col] {
            Position::Unvisited => {
                visited_positions.push((row,col));
                map[row][col] = Position::Visited(direction);
                guard_pos = (row, col);
                visit_count += 1;
            }
            Position::Visited(prev_dir) => {
                if prev_dir == direction {
                    loops_created += 1;
                    break;
                }

                guard_pos = (row, col);
            }
            Position::Obstructed => {
                direction =
                match direction {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                };
            }
        }
    }

    for row in map.iter() {
        for pos in row.iter() {
            match pos {
                Position::Unvisited => {
                    print!(".");
                }
                Position::Visited(dir) => {
                    print!("X");
                }
                Position::Obstructed => {
                    print!("#");
                }
            }
        }
        print!("\n");
    }

    println!("visit count {}", visit_count);

    for test in visited_positions.iter() {
        let mut map = original_map.clone();
        let mut direction = Direction::Up;
        let mut guard_pos = original_guard_pos;

        map[test.0][test.1] = Position::Obstructed;

        loop {
            let (mut row, mut col) = guard_pos;

            match direction {
                Direction::Down => {
                    if row == map.len() - 1 { break; }
                    row += 1;
                }
                Direction::Left => {
                    if col == 0 { break; }
                    col -= 1;
                }
                Direction::Up => {
                    if row == 0 { break; }
                    row -= 1;
                }
                Direction::Right => {
                    if col == map[0].len() - 1 { break; }
                    col += 1;
                }
            }

            match map[row][col] {
                Position::Unvisited => {
                    map[row][col] = Position::Visited(direction);
                    guard_pos = (row, col);
                    visit_count += 1;
                }
                Position::Visited(prev_dir) => {
                    if prev_dir == direction {
                        loops_created += 1;
                        for row in map.iter() {
                            for pos in row.iter() {
                                match pos {
                                    Position::Unvisited => {
                                        print!(".");
                                    }
                                    Position::Visited(dir) => {
                                        print!("X");
                                    }
                                    Position::Obstructed => {
                                        print!("#");
                                    }
                                }
                            }
                            print!("\n");
                        }
                        break;
                    }

                    guard_pos = (row, col);
                }
                Position::Obstructed => {
                    direction =
                    match direction {
                        Direction::Up => Direction::Right,
                        Direction::Right => Direction::Down,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Up,
                    };
                }
            }
        }
    }

    println!("part2: {}", loops_created);
}
