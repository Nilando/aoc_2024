use std::collections::HashSet;
use rand::{thread_rng, rngs::ThreadRng, Rng};


#[derive(Debug)]
struct Comp {
    a: usize,
    b: usize,
    c: usize,
    ip: usize,
}

#[derive(Debug)]
struct Instr {
    opcode: u8,
    operand: u8,
}

impl Comp {
    fn execute(&mut self, instrs: &[Instr], 
               program: &Vec<u8> // comment this for part1
   ) -> Vec<u8> {
        let mut output = vec![];
        while self.ip < instrs.len() {
            //println!("a : {}", self.a);
            let inst = &instrs[self.ip];

           // println!("A {:0>25b} : B {:0>25b} : C {:0>25b} : ip {}", self.a, self.b, self.c, self.ip);

            match inst.opcode {
                0 => {
                    let num = self.a;
                    let dem = 2_usize.pow(self.combo_value(inst.operand) as u32);
                    //println!("adv : {:b} = {:b} / {:b}", num / dem, num, dem);
                    self.a = num / dem;
                }
                1 => {
                    //println!("bxl : B{:b} = B{:b} ^ {:b}", self.b ^ inst.operand as usize, self.b, inst.operand);
                    self.b ^= inst.operand as usize;
                }
                2 => {
                    //println!("bst : {:b} = B{:b} % 8", self.combo_value(inst.operand) % 8, self.combo_value(inst.operand));
                    self.b = self.combo_value(inst.operand) % 8;
                }
                3 => {
                    //println!("jnz : {} => {}", self.a, inst.operand);
                    if self.a != 0 {
                        self.ip = inst.operand as usize;
                        continue;
                    }
                }
                4 => {
                    //println!("bxc : B{:b} = B{:b} ^ C{:b}", self.b ^ self.c, self.b, self.c);
                    self.b ^= self.c
                }
                5 => {
                    let o = self.combo_value(inst.operand) % 8;
                    if o != program[output.len()] as usize {
                        break;
                    }
                    //println!("out : {:0>3b}", o);
                    output.push(o as u8);
                }
                6 => {
                    let num = self.a;
                    let dem = 2_usize.pow(self.combo_value(inst.operand) as u32);
                    //println!("bdv : B{:b} = A{:b} / {:b}", num / dem, num, dem);
                    self.b = num / dem;
                }
                7 => {
                    let num = self.a;
                    let dem = 2_usize.pow(self.combo_value(inst.operand) as u32);
                    //println!("cdv : C{:b} = A{:b} / {:b}", num / dem, num, dem);
                    self.c = num / dem;
                }
                _ => panic!("bad opcode"),
            }

            self.ip += 1;
        }

        output
    }

    fn combo_value(&self, n: u8) -> usize {
        match n {
            0..=3 => n.into(),
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("bad combo val"),
        }
    }
}

fn main() {
    let input = std::include_str!("../input.txt");
    //let input = std::include_str!("../test_b.txt");
    let mut computer = Comp {
        a: 0,
        b: 0,
        c: 0,
        ip: 0,
    };
    let mut instrs = vec![];
    let mut program: Vec<u8> = vec![];

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
            let instr = Instr {
                opcode: opcode.parse().unwrap(),
                operand: operand.parse().unwrap(),
            };
            instrs.push(instr);
            program.push(opcode.parse().unwrap());
            program.push(operand.parse().unwrap());
        }
        println!("{:#?}", instrs);
    }

    println!("computer: {:#?}", computer);

    let part1 = computer.execute(&instrs, &program);

    println!("part1: {:?}", part1);

    part2(&instrs, &program);
}

fn part2(instrs: &Vec<Instr>, program: &Vec<u8>) {
    let mut seeds = HashSet::from([0; 1]);
    let mut result = usize::MAX;

    for i in 0..16 {
        println!("seed_len: {i}");
        seeds = part2_inner(instrs, program, seeds, i, &mut result);
    }

    println!("result: {}", result);
}

fn part2_inner(instrs: &Vec<Instr>, program: &Vec<u8>, seeds: HashSet<usize>, seed_len: usize, result: &mut usize) -> HashSet<usize> {
    let mut rng = thread_rng();
    let mut new_seeds = HashSet::new();

    for seed in seeds.iter() {
        for _ in 0..1000 {
            let a = gen_possible_a(&mut rng, *seed, seed_len) as usize;
            let mut computer = Comp {
                a,
                b: 0,
                c: 0,
                ip: 0,
            };

            let output = computer.execute(&instrs, &program);

            if output.len() == 16 {
                if a < *result {
                    *result = a;
                }
            }

            if output.len() >= seed_len {
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
