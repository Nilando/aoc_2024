use std::collections::HashMap;

fn look_at_stone(stone: u128, blinks: usize, total_blinks: usize, cache: &mut HashMap<(u128, usize), usize>) -> usize {
    if blinks == total_blinks {
        return 1;
    }

    if let Some(count) = cache.get(&(stone, total_blinks - blinks)) {
        return *count;
    }

    let count = 
        if stone == 0 {
            look_at_stone(1, blinks + 1, total_blinks, cache)
        } else {
            let string = stone.to_string();
            let s = string.as_str();

            if s.len() % 2 == 0 {
                let a = &s[0..s.len() / 2];
                let b = &s[s.len() / 2..s.len()];
                let mut count = 0;

                count += look_at_stone(a.parse().unwrap(), blinks + 1, total_blinks, cache);
                count += look_at_stone(b.parse().unwrap(), blinks + 1, total_blinks, cache);

                count
            } else {
                look_at_stone(stone * 2024, blinks + 1, total_blinks, cache)
            }
        };

    cache.insert((stone, total_blinks - blinks), count);

    return count;
}

fn main() {
    let stones: Vec<u128> = vec![8435, 234, 928434, 14, 0, 7, 92446, 8992692];
    let mut cache: HashMap<(u128, usize), usize> = HashMap::new();
    let mut part1 = 0;
    let mut part2 = 0;

    for stone in stones.iter() {
        part1 += look_at_stone(*stone, 0, 25, &mut cache);
        part2 += look_at_stone(*stone, 0, 75, &mut cache);
    }

    println!("part1: {:?}", part1);
    println!("part2: {:?}", part2);
}
