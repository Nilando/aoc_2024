use std::collections::{HashSet, HashMap};

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let mut part1 = HashSet::<(isize, isize)>::new();
    let mut part2 = HashSet::<(isize, isize)>::new();
    let mut antennas = HashMap::<char, Vec<(isize, isize)>>::new();

    let mut cols: isize = 0;
    let mut rows: isize = 0;

    for (y, line) in input.lines().enumerate() {
        cols = line.len() as isize;
        for (x, char) in line.chars().enumerate() {
            if char != '.' {
                match antennas.get_mut(&char) {
                    Some(locations) => {
                        locations.push((x as isize, y as isize));
                    }
                    None => {
                        antennas.insert(char, vec![(x as isize, y as isize)]);
                    }
                }
            }
        }
        rows += 1;
    }

    for (_, locations) in antennas.iter() {
        for i in 0..(locations.len() - 1) {
            for k in (i+1)..locations.len() {
                let (x1, y1) = locations[i];
                let (x2, y2) = locations[k];

                let x_diff = x1 - x2;
                let y_diff = y1 - y2;

                let mut d = 0;
                loop {
                    let (an_x, an_y) = (x1 + (x_diff * d), y1 + (y_diff * d));
                    if an_x < 0 || an_y < 0 || an_x >= cols || an_y >= rows {
                        break;
                    } else {
                        if d == 1 {
                            part1.insert((an_x, an_y));
                        }
                        part2.insert((an_x, an_y));
                    }
                    d += 1;
                }

                let mut d = 0;
                loop {
                    let (an_x, an_y) = (x2 - (x_diff * d), y2 - (y_diff * d));
                    if an_x < 0 || an_y < 0 || an_x >= cols || an_y >= rows {
                        break;
                    } else {
                        if d == 1 {
                            part1.insert((an_x, an_y));
                        }
                        part2.insert((an_x, an_y));
                    }
                    d += 1;
                }
            }
        }
    }

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '.' {
                if part2.get(&(x as isize, y as isize)).is_some() {
                    print!("#");
                } else {
                    print!(".");
                }
            } else {
                print!("{}", char);
            }
        }
        print!("\n");
    }

    println!("part1: {}", part1.len());
    println!("part2: {}", part2.len());
}
