use regex::Regex;

const PART2_ADDITION: u128 = 10000000000000;

type Machine = (u128, u128, u128, u128, u128, u128);

fn main() {
    let machines = parse_machines();
    let part1 = solve_machines(&machines, Some(100), 0);
    let part2 = solve_machines(&machines, None, PART2_ADDITION);

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

fn parse_machines() -> Vec<Machine>
{
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let re = Regex::new(r"(?m)^Button A: X\+([0-9]+), Y\+([0-9]+)$\nButton B: X\+([0-9]+), Y\+([0-9]+)$\nPrize: X=([0-9]+), Y=([0-9]+)").unwrap();
    let mut machines: Vec<Machine> = vec![];

    for (_, [ax, ay, bx, by, px, py]) in re.captures_iter(&input).map(|c| c.extract()) {
        machines.push((
            ax.parse().unwrap(), 
            ay.parse().unwrap(), 
            bx.parse().unwrap(), 
            by.parse().unwrap(), 
            px.parse().unwrap(),
            py.parse().unwrap(),
        ));
    }

    machines
}

fn solve_machines(machines: &[Machine], press_limit: Option<u128>, prize_adjustment: u128) -> u128 {
    let mut result = 0;

    for (ax, ay, bx, by, px, py) in machines.into_iter() {
        if let Some(tokens) = solve_machine(*ax, *ay, *bx, *by, px + prize_adjustment, py + prize_adjustment, press_limit) {
            result += tokens;
        }
    }

    return result;
}

fn solve_machine(ax: u128, ay: u128, bx: u128, by: u128, px: u128, py: u128, press_limit: Option<u128>) -> Option<u128> {
    let mut ap = 0;
    let mut bp = 0;

    loop {
        ap = (px - (bp * bx)) / ax;

        if (bp * by) + (ap * ay) > py {
            break;
        }

        if (bp * bx) + (ap * ax) == px
        && (bp * by) + (ap * ay) == py {
            if let Some(l) = press_limit {
                if bp > l || ap > l {
                    break;
                }
            }
            return Some((ap * 3) + bp);
        }

        let d = ((py - (bp * by)) - (ap * ay)) / by;
        if d == 0 {
            bp += 1;
        }
        bp += d;
        if bp * bx > px {
            break;
        }
    }

    ap = 0;
    bp = 0;

    loop {
        bp = (px - (ap * ax)) / bx;

        if (bp * by) + (ap * ay) > py {
            break;
        }

        if (bp * bx) + (ap * ax) == px
        && (bp * by) + (ap * ay) == py {
            if let Some(l) = press_limit {
                if bp > l || ap > l {
                    break;
                }
            }
            return Some((ap * 3) + bp);
        }

        let d = ((py - (bp * by)) - (ap * ay)) / ay;
        if d == 0 {
            ap += 1;
        }
        ap += d;

        if ap * ax > px {
            break;
        }

    }

    None
}
