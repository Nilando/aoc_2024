fn main() {
    let input = std::include_bytes!("../input.txt");

    let mut data = vec![];
    let mut move_file_id = 0;

    for (i, x) in input.iter().enumerate() {
        if *x == b'\n' {
            break;
        }

        let n = x - b'0';
        if i % 2 == 0 {
            for _ in 0..n {
                data.push(Some(i/2));
                move_file_id = i/2;
            }
        } else {
            for _ in 0..n {
                data.push(None);
            }
        }
    }

    let mut part1_data = data.clone();

    let mut a = 0;
    let mut b = part1_data.len() - 1;

    while a < b {
        if part1_data[a].is_some() {
            a += 1;
            continue;
        }

        if part1_data[b].is_none() {
            b -= 1;
            continue;
        }

        part1_data.swap(a, b);
        a += 1;
        b -= 1;
    }

    for fid in (0..=move_file_id).rev() {
        let (file_start, size) = find_file(&data, fid);

        if let Some(space_start) = find_space(&data, file_start, size) {
            for i in 0..size {
                data.swap(file_start + i, space_start + i);
            }
        }
    }


    let mut part1 = 0;
    for (i, d) in part1_data.iter().enumerate() {
        if let Some(x) = d {
            part1 += i * x;
        }
    }

    println!("part1: {part1}");


    let mut part2 = 0;
    for (i, d) in data.iter().enumerate() {
        if let Some(x) = d {
            part2 += i * x;
        }
    }

    println!("part2: {part2}");
}

fn print(data: &Vec<Option<usize>>) {
    for i in data.iter() {
        if let Some(x) = i {
            print!("{x}");
        } else {
            print!(".");
        }
    }

    println!("");
}

fn find_space(data: &Vec<Option<usize>>, file_start: usize, size: usize) -> Option<usize> {
    let mut current_hole = 0;
    let mut hole_start = 0;
    for (i, id) in data.iter().enumerate() {
        if i >= file_start {
            return None;
        }

        if id.is_none() {
            if current_hole == 0 {
                current_hole += 1;
                hole_start = i;
            } else {
                current_hole += 1;
            }
        } else {
            current_hole = 0;
        }

        if current_hole >= size {
            return Some(hole_start);
        }
    }

    None
}

fn find_file(data: &Vec<Option<usize>>, fid: usize) -> (usize, usize) {
    let mut size = 0;
    for (i, id) in data.iter().enumerate().rev() {
        if (id.is_none() || id.unwrap() != fid) && size != 0 {
            return (i + 1, size);
        }

        if let Some(o) = id {
            if fid == *o {
                size += 1;
            }
        }

    }

    if size != 0 {
        return (0, size);
    }

    panic!("unable to find file {fid}");
}
