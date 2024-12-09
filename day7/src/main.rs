fn main() {
    part1();
    part2();
}

fn part1() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let mut part1: u128 = 0;

    for line in input.lines() {
        let raw_equation: Vec<&str> = line.split(':').collect();
        let target: usize = raw_equation[0].parse().unwrap();
        let operands: Vec<usize> = raw_equation[1].trim().split(' ').map(|s| s.parse().unwrap()).collect();

        for i in 0..2_usize.pow(operands.len() as u32 - 1) {
            let mut sum = operands[0];

            for (idx, operand) in operands.iter().skip(1).enumerate() {
                if sum > target {
                    break;
                }

                if (i & (1 << idx)) != 0 {
                    sum += operand;
                } else {
                    sum *= operand;
                }
            }

            if sum == target {
                println!("target: {}", target);
                println!("operands: {:?}", operands);
                println!("i: {:0<8b}", i);
                part1 += target as u128;
                break;
            }
        }
    }

    println!("Part 1: {}", part1);
}

fn part2() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let mut part2: u128 = 0;

    for line in input.lines() {
        let raw_equation: Vec<&str> = line.split(':').collect();
        let target: usize = raw_equation[0].parse().unwrap();
        let operands: Vec<usize> = raw_equation[1].trim().split(' ').map(|s| s.parse().unwrap()).collect();

        for i in 0..3_usize.pow(operands.len() as u32 - 1) {
            let mut sum = operands[0];
            let mut x = i;

            for operand in operands.iter().skip(1) {
                if sum > target {
                    break;
                }

                match x % 3 {
                    0 => sum += operand,
                    1 => sum *= operand,
                    2 => {
                        let mut a = sum.to_string(); 
                        let b = operand.to_string();
                        a.push_str(b.as_str());

                        sum = a.parse().unwrap();
                    },
                    _ => panic!("bad"),
                }

                x -= x % 3;
                x = x / 3;
            }

            if sum == target {
                part2 += target as u128;
                break;
            }
        }
    }

    println!("Part 2: {}", part2);
}
