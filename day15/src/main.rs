
fn main() {
    part1::part1();
    part2::part2();
}

mod part2 {
    #[derive(Debug, PartialEq, Clone, Copy)]
    enum Space {
        LeftBox,
        RightBox,
        Wall,
        Empty,
    }

    pub fn part2() {
        let (mut warehouse, moves, robot) = parse_input();

        move_robot(&mut warehouse, &moves, robot);

        let mut part2 = 0;
        for (row, spaces) in warehouse.iter().enumerate() {
            for (col, space) in spaces.iter().enumerate() {
                if *space == Space::LeftBox {
                    part2 += (row * 100) + col;
                }
            }
        }

        println!("part2 {}", part2);
    }

    fn move_robot(warehouse: &mut Vec<Vec<Space>>, moves: &[char], mut robot: (usize, usize)) {
        for m in moves.iter() {
            let direction: (isize, isize) =
            match m {
                '^' => (-1, 0),
                'v' => (1, 0),
                '<' => (0, -1),
                '>' => (0, 1),
                _ => panic!("bad move"),
            };

            //print_warehouse(&warehouse, robot);
            //println!("move: {m}");

            let move_coords = ((robot.0 as isize + direction.0) as usize, (robot.1 as isize + direction.1) as usize);
            let move_space = &warehouse[move_coords.0][move_coords.1];

            if *move_space == Space::Wall {
                // do nothing
            } else if *move_space == Space::Empty {
                robot = move_coords;
            } else {
                if direction.0 == 0 {
                    if move_box_horizontal(warehouse, direction, move_coords) {
                        robot = move_coords;
                    }
                } else {
                    let box_coords =
                    if *move_space == Space::RightBox {
                        (move_coords.0, move_coords.1 - 1)
                    } else {
                        move_coords
                    };

                    if can_move_box_vertical(warehouse, direction, box_coords) {
                        move_box_vertical(warehouse, direction, box_coords);
                        robot = move_coords;
                    }
                }
            }
        }
    }

    // box coords represent the leftbox cords
    fn move_box_horizontal(warehouse: &mut Vec<Vec<Space>>, direction: (isize, isize), box_coords: (usize, usize)) -> bool {
        let move_coords = (box_coords.0, (box_coords.1 as isize + direction.1) as usize);
        let move_space = &mut warehouse[move_coords.0][move_coords.1];
        if *move_space == Space::Wall {
            return false;
        }

        if *move_space == Space::Empty || move_box_horizontal(warehouse, direction, move_coords) {
            warehouse[move_coords.0][move_coords.1] = warehouse[box_coords.0][box_coords.1];
            warehouse[box_coords.0][box_coords.1] = Space::Empty;
            return true;
        }

        return false;
    }

    fn move_box_vertical(warehouse: &mut Vec<Vec<Space>>, direction: (isize, isize), box_coords: (usize, usize)) {
        let left_move_coords = ((box_coords.0 as isize + direction.0) as usize, box_coords.1);
        let left_move_space = warehouse[left_move_coords.0][left_move_coords.1];
        let right_move_coords = ((box_coords.0 as isize + direction.0) as usize, box_coords.1 + 1);
        let right_move_space = warehouse[right_move_coords.0][right_move_coords.1];

        if left_move_space == Space::LeftBox {
            move_box_vertical(warehouse, direction, left_move_coords);
        }

        if left_move_space == Space::RightBox {
            move_box_vertical(warehouse, direction, (left_move_coords.0, left_move_coords.1 - 1));
        }

        if right_move_space == Space::LeftBox {
            move_box_vertical(warehouse, direction, right_move_coords);
        }

        warehouse[left_move_coords.0][left_move_coords.1] = Space::LeftBox;
        warehouse[right_move_coords.0][right_move_coords.1] = Space::RightBox;
        warehouse[box_coords.0][box_coords.1] = Space::Empty;
        warehouse[box_coords.0][box_coords.1 + 1] = Space::Empty;
    }

    fn can_move_box_vertical(warehouse: &Vec<Vec<Space>>, direction: (isize, isize), box_coords: (usize, usize)) -> bool {
        let left_move_coords = ((box_coords.0 as isize + direction.0) as usize, box_coords.1);
        let left_move_space = &warehouse[left_move_coords.0][left_move_coords.1];

        if *left_move_space == Space::Wall {
            return false;
        }

        if *left_move_space == Space::LeftBox {
            return can_move_box_vertical(warehouse, direction, left_move_coords);
        }

        let right_move_coords = ((box_coords.0 as isize + direction.0) as usize, box_coords.1 + 1);
        let right_move_space = &warehouse[right_move_coords.0][right_move_coords.1];

        if *right_move_space == Space::Wall {
            return false;
        }

        if *left_move_space == Space::Empty && *right_move_space == Space::LeftBox {
            return can_move_box_vertical(warehouse, direction, right_move_coords);
        }

        if *left_move_space == Space::RightBox && *right_move_space == Space::Empty {
            return can_move_box_vertical(warehouse, direction, (left_move_coords.0, left_move_coords.1 - 1));
        }

        if *left_move_space == Space::RightBox && *right_move_space == Space::LeftBox {
            return can_move_box_vertical(warehouse, direction, (left_move_coords.0, left_move_coords.1 -1)) && can_move_box_vertical(warehouse, direction, right_move_coords);
        }

        return true;
    }

    fn parse_input() -> (Vec<Vec<Space>>, Vec<char>, (usize, usize)) {
        //let input = std::include_str!("../test.txt");
        let input = std::include_str!("../input.txt");
        let mut moves: Vec<char> = vec![];
        let mut warehouse: Vec<Vec<Space>> = vec![];
        let mut parse_flag = false;
        let mut robot = (0, 0);

        for (r, line) in input.lines().enumerate() {
            if line.is_empty() {
                parse_flag = true;
                continue;
            }

            if parse_flag {
                for c in line.chars() {
                    moves.push(c);
                }
            } else {
                let mut row = vec![];
                for (c, s) in line.chars().enumerate() {
                    match s {
                        '#' => {
                            row.push(Space::Wall);
                            row.push(Space::Wall);
                        }
                        '.' => {
                            row.push(Space::Empty);
                            row.push(Space::Empty);
                        }
                        'O' => {
                            row.push(Space::LeftBox);
                            row.push(Space::RightBox);
                        }
                        '@' => {
                            row.push(Space::Empty);
                            row.push(Space::Empty);
                            robot = (r, c*2);
                        }
                        _ => panic!("parsing failed"),
                    }
                }
                warehouse.push(row);
            }
        }

        (warehouse, moves, robot)
    }

    fn print_warehouse(warehouse: &Vec<Vec<Space>>, robot: (usize, usize)) {
        for (row, spaces) in warehouse.iter().enumerate() {
            for (col, space) in spaces.iter().enumerate() {
                if row == robot.0 && col == robot.1 {
                    //assert!(*space == Space::Empty);
                    print!("@");
                    continue;
                }
                match space {
                    Space::Wall => print!("#"),
                    Space::Empty => print!("."),
                    Space::LeftBox => print!("["),
                    Space::RightBox => print!("]"),
                };
            }

            println!("");
        }
    }
}

mod part1 {
    #[derive(Debug, PartialEq)]
    enum Space {
        Box,
        Wall,
        Empty,
    }

    pub fn part1() {
        let input = std::include_str!("../input.txt");
        //let input = std::include_str!("../test.txt");

        let mut moves: Vec<char> = vec![];
        let mut warehouse: Vec<Vec<Space>> = vec![];
        let mut parse_flag = false;
        let mut robot = (0, 0);
        for (r, line) in input.lines().enumerate() {
            if line.is_empty() {
                parse_flag = true;
                continue;
            }

            if parse_flag {
                for c in line.chars() {
                    moves.push(c);
                }
            } else {
                let mut row = vec![];
                for (c, s) in line.chars().enumerate() {
                    match s {
                        '#' => row.push(Space::Wall),
                        '.' => row.push(Space::Empty),
                        'O' => row.push(Space::Box),
                        '@' => {
                            row.push(Space::Empty);
                            robot = (r, c);
                        }
                        _ => panic!("parsing failed"),
                    }
                }
                warehouse.push(row);
            }
        }

        //println!("robot: {}, {}", robot.0, robot.1);

        for m in moves.iter() {
            let direction: (isize, isize) =
            match m {
                '^' => (-1, 0),
                'v' => (1, 0),
                '<' => (0, -1),
                '>' => (0, 1),
                _ => panic!("bad move"),
            };

            //println!("move: {}", m);
            //println!("{:?}", direction);

            let move_space = &warehouse[(robot.0 as isize + direction.0) as usize][(robot.1 as isize + direction.1) as usize];

            //println!("move_space: {:?}", move_space);

            if *move_space == Space::Wall {
                // do nothing
            } else if *move_space == Space::Empty {
                robot = ((robot.0 as isize + direction.0) as usize, (robot.1 as isize + direction.1) as usize);
            } else {
                let mut i = 1;

                loop {
                    let box_move_space = &mut warehouse[(robot.0 as isize + (direction.0 * i)) as usize][(robot.1 as isize + (direction.1 * i)) as usize];
                    //println!("box move_space: {:?}", box_move_space);
                    //println!("{:?}", ((robot.0 as isize + (direction.0 * i)) as usize, (robot.1 as isize + (direction.1 * i)) as usize));
                    if *box_move_space == Space::Wall {
                        break;
                    }

                    if *box_move_space == Space::Empty {
                        *box_move_space = Space::Box;
                        warehouse[(robot.0 as isize + direction.0) as usize][(robot.1 as isize + direction.1) as usize] = Space::Empty;
                        robot = ((robot.0 as isize + direction.0) as usize, (robot.1 as isize + direction.1) as usize);
                        break;
                    }

                    i += 1;
                }
            }
            //print_warehouse(&warehouse, robot);
        }
        //print_warehouse(&warehouse, robot);

        let mut part1 = 0;
        for (row, spaces) in warehouse.iter().enumerate() {
            for (col, space) in spaces.iter().enumerate() {
                if *space == Space::Box {
                    part1 += (row * 100) + col;
                }
            }
        }

        println!("part1 {}", part1);
    }

    fn print_warehouse(warehouse: &Vec<Vec<Space>>, robot: (usize, usize)) {
        for (row, spaces) in warehouse.iter().enumerate() {
            for (col, space) in spaces.iter().enumerate() {
                if row == robot.0 && col == robot.1 {
                    assert!(*space == Space::Empty);
                    print!("@");
                    continue;
                }
                match space {
                    Space::Wall => print!("#"),
                    Space::Empty => print!("."),
                    Space::Box => print!("O"),
                };
            }

            println!("");
        }
    }
}
