use std::fmt::{Display, Debug};

struct DirPadState {
    hover: DirectionalKey,
}

impl DirPadState {
    fn new() -> Self {
        Self {
            hover: DirectionalKey::Press,
        }
    }

    fn press_key(&mut self, key: &DirectionalKey) -> Vec<DirectionalKey> {
        let instructions = self.hover.directional_instructions_to_press(&key);
        self.hover = *key;
        instructions
    }
}

struct DoorState {
    hover: NumericKey,
}

impl DoorState {
    fn new() -> Self {
        Self {
            hover: NumericKey::A,
        }
    }

    fn press_key(&mut self, key: NumericKey) -> Vec<DirectionalKey> {
        let instructions = self.hover.directional_instructions_to_press(&key);
        self.hover = key;
        instructions
    }
}

enum NumericKey {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A
}

impl NumericKey {
    fn new(c: char) -> Self {
        match c {
            '0' => Self::Zero,
            '1' => Self::One,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'A' => Self::A,
            _ => panic!("bad key"),
        }
    }

    fn directional_instructions_to_press(&self, this: &NumericKey) -> Vec<DirectionalKey> {
        let (start_row, start_col) = self.to_pos();
        let (end_row, end_col) = this.to_pos();

        let row_diff: isize = end_row - start_row;
        let col_diff: isize = end_col - start_col;

        //println!("row diff: {} col diff {}", row_diff, col_diff);

        let mut instructions = vec![];

        if start_col == 0 && end_row == 3 {
            for _ in 0..col_diff.abs() {
                instructions.push(DirectionalKey::Right);
            }

            for _ in 0..row_diff.abs() {
                instructions.push(DirectionalKey::Down);
            }
        } else if start_row == 3 && end_col == 0 {
            for _ in 0..row_diff.abs() {
                instructions.push(DirectionalKey::Up);
            }

            for _ in 0..col_diff.abs() {
                instructions.push(DirectionalKey::Left);
            }
        } else if col_diff < 0 {
            for _ in 0..col_diff.abs() {
                instructions.push(DirectionalKey::Left);
            }

            if row_diff > 0 {
                for _ in 0..row_diff.abs() {
                    instructions.push(DirectionalKey::Down);
                }
            } else {
                for _ in 0..row_diff.abs() {
                    instructions.push(DirectionalKey::Up);
                }
            }
        } else {
            if row_diff > 0 {
                for _ in 0..row_diff.abs() {
                    instructions.push(DirectionalKey::Down);
                }
            } else {
                for _ in 0..row_diff.abs() {
                    instructions.push(DirectionalKey::Up);
                }
            }

            for _ in 0..col_diff.abs() {
                instructions.push(DirectionalKey::Right);
            }
        }

        instructions.push(DirectionalKey::Press);

        instructions
    }

    fn to_pos(&self) -> (isize, isize) {
        match self {
            Self::Seven => (0, 0),
            Self::Eight => (0, 1),
            Self::Nine => (0, 2),
            Self::Four => (1, 0),
            Self::Five => (1, 1),
            Self::Six => (1, 2),
            Self::One => (2, 0),
            Self::Two => (2, 1),
            Self::Three => (2, 2),
            Self::Zero => (3, 1),
            Self::A => (3, 2),
        }
    }
}

impl Display for NumericKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Zero => write!(f, "0"),
            Self::One => write!(f, "1"),
            Self::Two => write!(f, "2"),
            Self::Three => write!(f, "3"),
            Self::Four => write!(f, "4"),
            Self::Five => write!(f, "5"),
            Self::Six => write!(f, "6"),
            Self::Seven => write!(f, "7"),
            Self::Eight => write!(f, "8"),
            Self::Nine => write!(f, "9"),
            Self::A => write!(f, "A"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum DirectionalKey {
    Left,
    Down,
    Right,
    Up,
    Press,
}

impl DirectionalKey {
    fn directional_instructions_to_press(&self, this: &DirectionalKey) -> Vec<DirectionalKey> {
        let (start_row, start_col) = self.to_pos();
        let (end_row, end_col) = this.to_pos();

        let row_diff: isize = end_row - start_row;
        let col_diff: isize = end_col - start_col;

        let mut instructions = vec![];

        if end_row == 0  && start_col == 0 {
            for _ in 0..col_diff.abs() {
                instructions.push(DirectionalKey::Right);
            }

            for _ in 0..row_diff.abs() {
                instructions.push(DirectionalKey::Up);
            }
        } else if end_col == 0 {
            for _ in 0..row_diff.abs() {
                instructions.push(DirectionalKey::Down);
            }

            for _ in 0..col_diff.abs() {
                instructions.push(DirectionalKey::Left);
            }
        } else if col_diff < 0 {
            for _ in 0..col_diff.abs() {
                instructions.push(DirectionalKey::Left);
            }

            if row_diff > 0 {
                for _ in 0..row_diff.abs() {
                    instructions.push(DirectionalKey::Down);
                }
            } else {
                for _ in 0..row_diff.abs() {
                    instructions.push(DirectionalKey::Up);
                }
            }
        } else {
            if row_diff > 0 {
                for _ in 0..row_diff.abs() {
                    instructions.push(DirectionalKey::Down);
                }
            } else {
                for _ in 0..row_diff.abs() {
                    instructions.push(DirectionalKey::Up);
                }
            }

            for _ in 0..col_diff.abs() {
                instructions.push(DirectionalKey::Right);
            }
        }

        instructions.push(DirectionalKey::Press);

        instructions
    }

    fn to_pos(&self) -> (isize, isize) {
        match self {
            Self::Up => (0, 1),
            Self::Press => (0, 2),
            Self::Left => (1, 0),
            Self::Down => (1, 1),
            Self::Right => (1, 2),
        }
    }
}

impl Display for DirectionalKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left => write!(f, "<"),
            Self::Right => write!(f, ">"),
            Self::Down => write!(f, "v"),
            Self::Up => write!(f, "^"),
            Self::Press => write!(f, "A"),
        }
    }
}

impl Debug for DirectionalKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left => write!(f, "<"),
            Self::Right => write!(f, ">"),
            Self::Down => write!(f, "v"),
            Self::Up => write!(f, "^"),
            Self::Press => write!(f, "A"),
        }
    }
}

use std::collections::HashMap;

fn robot_chain(dir_keys: &Vec<DirectionalKey>, chain_len: usize, cache: &mut HashMap<(usize, Vec<DirectionalKey>), usize>) -> usize {
    let mut dp = DirPadState::new();
    let mut result = 0;

    for dir_key in dir_keys.into_iter() {
        let nested_dir_keys = dp.press_key(dir_key);

        if chain_len == 1 {
            result += nested_dir_keys.len();
        } else {
            let cache_entry = (chain_len, nested_dir_keys);

            if let Some(n) = cache.get(&cache_entry) {
                result += n
            } else {
                let n = robot_chain(&cache_entry.1, chain_len - 1, cache);

                result += n;
                cache.insert(cache_entry, n);
            }
        }
    }


    result
}

fn main() {
    let input = std::include_str!("../input.txt");
    //let input = std::include_str!("../test.txt");
    let mut part1 = 0;
    let mut part2 = 0;

    for line in input.lines() {
        let mut door_state = DoorState::new();
        let code = line.trim();
        let mut p1 = 0;
        let mut p2 = 0;
        let mut cache = HashMap::new();

        for c in code.chars() {
            let num_key = NumericKey::new(c);
            let dir_keys = door_state.press_key(num_key); 

            p1 += robot_chain(&dir_keys, 2, &mut cache);
            p2 += robot_chain(&dir_keys, 25, &mut cache);
        }


        let num = &code[0..code.len() - 1];

        part1 += p1 * num.parse::<usize>().unwrap();
        part2 += p2 * num.parse::<usize>().unwrap();
    }

    println!("part1: {part1}");
    println!("part2: {part2}");
}
