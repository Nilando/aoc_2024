use std::collections::HashSet;
use rand::{thread_rng, rngs::ThreadRng, Rng};


#[derive(Debug)]
struct Comp {
    a: usize,
    b: usize,
    c: usize,
}

impl Comp {
    fn execute_optimized(&mut self, program: &Vec<usize>, must_match: bool) -> Vec<usize> {
        let mut output = vec![];
        let mut a = self.a;
        let mut b;
        let mut c;

        while a != 0 {
            b = a % 8;
            b ^= 1;
            c = a / 2_usize.pow(b as u32);
            b ^= 5;
            b ^= c;
            b = b % 8;

            if must_match && program[output.len()] != b {
                return output;
            }

            output.push(b);
            a /= 8;
        }

        output
    }
}

fn part2(program: &Vec<usize>) {
    let mut rng = thread_rng();
    let mut seeds = HashSet::from([0; 1]);
    let mut result = usize::MAX;

    for i in 0..16 {
        seeds = part2_inner(&mut rng, program, seeds, i, &mut result);
    }

    println!("part2: {}", result);
}

fn part2_inner(rng: &mut ThreadRng, program: &Vec<usize>, seeds: HashSet<usize>, seed_len: usize, result: &mut usize) -> HashSet<usize> {
    let mut new_seeds = HashSet::new();

    for seed in seeds.iter() {
        for _ in 0..500 {
            let a = gen_possible_a(rng, *seed, seed_len) as usize;
            let mut computer = Comp { a, b: 0, c: 0 };
            let output = computer.execute_optimized(program, true);

            if output.len() == 16 {
                if a < *result {
                    *result = a;
                }

                continue;
            }

            if output.len() > seed_len {
                new_seeds.insert(a % 8_usize.pow((seed_len + 1) as u32));
            }
        }
    }

    new_seeds
}

fn gen_possible_a(rng: &mut ThreadRng, seed: usize, seed_len: usize) -> usize {
    let mut a = seed;

    for i in seed_len..16 {
        let t = rng.gen_range(0..8) << (i * 3);

        a += t;
    }

    a
}

fn main() {
    let input = std::include_str!("../input.txt");
    let mut computer = Comp { a: 0, b: 0, c: 0 };
    let mut program: Vec<usize> = vec![];

    for (i, line) in input.lines().enumerate() {
        let mut iter = line.split(':');
        iter.next();

        if i == 0 {
            computer.a = iter.next().unwrap().trim().parse().unwrap();
            continue;
        }
        if i == 1 {
            computer.b = iter.next().unwrap().trim().parse().unwrap();
            continue;
        }
        if i == 2 {
            computer.c = iter.next().unwrap().trim().parse().unwrap();
            continue;
        }
        if i == 3 {
            continue
        }

        let mut iter = iter.next().unwrap().trim().split(',');

        while let Some(opcode) = iter.next() {
            let operand = iter.next().unwrap();

            program.push(opcode.parse().unwrap());
            program.push(operand.parse().unwrap());
        }
        //println!("{:#?}", instrs);
    }

    //println!("computer: {:#?}", computer);

    let part1 = computer.execute_optimized(&program, false);

    println!("part1: {:?}", part1);

    part2(&program);
}
