#[inline]
unsafe fn parse_num(input: &mut *const u8) -> i8 {
    let a = (**input) - b'0';
    if *(*input).add(1) < b'0' {
        *input = (*input).add(2);
        return a as i8;
    }
    let b = *(*input).add(1) - b'0';
    if *(*input).add(2) < b'0' {
        *input = (*input).add(3);
        return ((a * 10) + b) as i8;
    }

    let c = *(*input).add(2) - b'0';
    *input = (*input).add(4);

    return ((a * 100) + (b * 10) + c) as i8;
}

#[inline]
unsafe fn parse_velocity(input: &mut *const u8) -> i8 {
    let negative = (**input) == b'-';
    if negative {
        *input = (*input).add(1);
    }

    if negative {
        parse_num(input) as i8 * -1
    } else {
        parse_num(input) as i8
    }
}

use std::collections::HashMap;

fn main() {

    unsafe {
        //let input_str = std::include_bytes!("../test.txt"); 
        //let width = 11;
        //let height = 7;
        for i in 0..10000 {
            let mut robots: HashMap<(u8, u8), usize> = HashMap::new();
            let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
            let input_str = std::include_bytes!("../input.txt"); 
            let width = 101;
            let height = 103;
            let seconds = i;
            let mut input = input_str as *const u8;
            let end = input.add(input_str.len());

            loop {
                if input == end {
                    break;
                }

                input = input.add(2);
                let x = parse_num(&mut input);
                let y = parse_num(&mut input);

                input = input.add(2);
                let xv = parse_velocity(&mut input);
                let yv = parse_velocity(&mut input);

                let mut nx = (x as isize + (xv as isize * seconds)) % width;
                let mut ny = (y as isize + (yv as isize * seconds)) % height;

                if nx < 0 {
                    nx = width + nx;
                }

                if ny < 0 {
                    ny = height + ny;
                }

                if nx == width / 2 || ny == height / 2 {
                    continue;
                }

                /*
                println!("x: {}", x);
                println!("y: {}", y);
                println!("xv: {}", xv);
                println!("yv: {}", yv);
                println!("{nx}, {ny}");
                */

                if let Some(c) =  robots.get(&(nx as u8, ny as u8)) {
                    robots.insert((nx as u8, ny as u8), c + 1);
                } else {
                    robots.insert((nx as u8, ny as u8), 1);
                }

                if nx < width / 2  && ny < height / 2 {
                    q1 += 1;
                } else if nx > width / 2  && ny < height / 2 {
                    q2 += 1;
                } else if nx < width / 2  && ny > height / 2 {
                    q3 += 1;
                } else {
                    q4 += 1;
                }
            }

            let mut tree_flag = false;
            for row in 0..height {
                let mut x = 0;
                for col in 0..width {
                    if let Some(c) = robots.get(&(col as u8, row as u8)) {
                        x += 1;
                        if x > 10 {
                            tree_flag = true;
                        }
                    } else {
                        x = 0;
                    }
                }
            }

            if !tree_flag {
                continue;
            }

            for row in 0..height {
                for col in 0..width {
                    if let Some(c) = robots.get(&(col as u8, row as u8)) {
                        print!("{c}");
                    } else {
                        print!(".");
                    }
                }
                print!("\n");
            }

            println!("");
            println!("===========  {i}  ============");
            println!("");


            //println!("{} {} {} {}", q1, q2, q3, q4);
            //println!("part1: {}", q1 * q2 * q3 * q4);
        }
    }
}
