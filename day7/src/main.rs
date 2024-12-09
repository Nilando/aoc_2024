fn is_valid_part1(target: usize, sum: usize, nums: &[usize]) -> bool {
    if nums.is_empty() {
        return sum == target;
    }

    if sum > target {
        return false
    }

    let operand = nums.first().unwrap();

    is_valid_part1(target, sum + operand, &nums[1..nums.len()])
    || is_valid_part1(target, sum * operand, &nums[1..nums.len()])
}

fn is_valid_part2(target: usize, sum: usize, nums: &[usize]) -> bool {
    if nums.is_empty() {
        return sum == target;
    }

    if sum > target {
        return false
    }

    let operand = nums.first().unwrap();

    is_valid_part2(target, sum + operand, &nums[1..nums.len()])
    || is_valid_part2(target, sum * operand, &nums[1..nums.len()])
    || is_valid_part2(target, format!("{}{}", sum, operand).parse().unwrap(), &nums[1..nums.len()])
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let mut part1 = 0;
    let mut part2 = 0;

    for line in input.lines() {
        let raw_equation: Vec<&str> = line.split(':').collect();
        let target: usize = raw_equation[0].parse().unwrap();
        let operands: Vec<usize> = raw_equation[1].trim().split(' ').map(|s| s.parse().unwrap()).collect();

        if is_valid_part1(target, operands[0], &operands[1..operands.len()]) {
            part1 += target;
            part2 += target;
        } else if is_valid_part2(target, operands[0], &operands[1..operands.len()]) {
            part2 += target;
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
