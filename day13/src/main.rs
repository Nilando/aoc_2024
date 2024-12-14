use regex::Regex;

const C: u128 = 10000000000000;

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let re = Regex::new(r"(?m)^Button A: X\+([0-9]+), Y\+([0-9]+)$\nButton B: X\+([0-9]+), Y\+([0-9]+)$\nPrize: X=([0-9]+), Y=([0-9]+)").unwrap();
    let mut machines: Vec<(u128, u128, u128, u128, u128, u128)> = vec![];
    let mut part1 = 0_u128;

    for (_, [ax, ay, bx, by, px, py]) in re.captures_iter(&input).map(|c| c.extract()) {
        machines.push((
            ax.parse().unwrap(), 
            ay.parse().unwrap(), 
            bx.parse().unwrap(), 
            by.parse().unwrap(), 
            px.parse::<u128>().unwrap() + C, 
            py.parse::<u128>().unwrap() + C, 
        ));
    }

    for (ax, ay, bx, by, px, py) in machines.into_iter() {
        let mut ap = 0;
        let mut bp = 0;
        let mut flag = false;

        loop {
            ap = (px - (bp * bx)) / ax;

            if (bp * by) + (ap * ay) > py {
                break;
            }

            if (bp * bx) + (ap * ax) == px
            && (bp * by) + (ap * ay) == py {
                part1 += (ap * 3) + bp;
                flag = true;
                break;
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

        if flag { continue };
        ap = 0;
        bp = 0;

        loop {
            bp = (px - (ap * ax)) / bx;

            if (bp * by) + (ap * ay) > py {
                break;
            }

            if (bp * bx) + (ap * ax) == px
            && (bp * by) + (ap * ay) == py {
                part1 += (ap * 3) + bp;
                break;
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
    }

    println!("part1: {}", part1);
}
