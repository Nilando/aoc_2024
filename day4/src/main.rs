fn part1() {
    let file = std::fs::read_to_string("./input.txt").unwrap();
    let mut grid: Vec<Vec<char>> = vec![];
    for line in file.lines() {
        grid.push(line.chars().collect());
    }

    let rows: usize = grid.len();
    let cols: usize = grid[0].len();
    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            let c = grid[row][col];

            if c == 'X' {
                if col + 3 < cols {
                    if grid[row][col + 1] == 'M'
                    && grid[row][col + 2] == 'A'
                    && grid[row][col + 3] == 'S' {
                        count += 1;
                    }
                }

                if row + 3 < rows {
                    if grid[row + 1][col] == 'M'
                    && grid[row + 2][col] == 'A'
                    && grid[row + 3][col] == 'S' {
                        count += 1;
                    }
                }

                if row + 3 < rows && col + 3 < cols{
                    if grid[row + 1][col + 1] == 'M'
                    && grid[row + 2][col + 2] == 'A'
                    && grid[row + 3][col + 3] == 'S' {
                        count += 1;
                    }
                }

                if row + 3 < rows && col >= 3 {
                    if grid[row + 1][col - 1] == 'M'
                    && grid[row + 2][col - 2] == 'A'
                    && grid[row + 3][col - 3] == 'S' {
                        count += 1;
                    }
                }
            }

            if c == 'S' {
                if col + 3 < cols {
                    if grid[row][col + 1] == 'A'
                    && grid[row][col + 2] == 'M'
                    && grid[row][col + 3] == 'X' {
                        count += 1;
                    }
                }

                if row + 3 < rows {
                    if grid[row + 1][col] == 'A'
                    && grid[row + 2][col] == 'M'
                    && grid[row + 3][col] == 'X' {
                        count += 1;
                    }
                }

                if row + 3 < rows && col + 3 < cols{
                    if grid[row + 1][col + 1] == 'A'
                    && grid[row + 2][col + 2] == 'M'
                    && grid[row + 3][col + 3] == 'X' {
                        count += 1;
                    }
                }

                if row + 3 < rows && col >= 3 {
                    if grid[row + 1][col - 1] == 'A'
                    && grid[row + 2][col - 2] == 'M'
                    && grid[row + 3][col - 3] == 'X' {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("part1: {}", count);
}
fn part2() {
    let file = std::fs::read_to_string("./input.txt").unwrap();
    let mut grid: Vec<Vec<char>> = vec![];
    for line in file.lines() {
        grid.push(line.chars().collect());
    }

    let rows: usize = grid.len();
    let cols: usize = grid[0].len();
    let mut count = 0;

    for row in 1..(rows - 1) {
        for col in 1..(cols - 1) {
            let c = grid[row][col];

            if c == 'A' {
                if grid[row + 1][col - 1] == 'M'
                && grid[row - 1][col - 1] == 'M'
                && grid[row + 1][col + 1] == 'S'
                && grid[row - 1][col + 1] == 'S' {
                    count += 1;
                }

                if grid[row + 1][col - 1] == 'S'
                && grid[row - 1][col - 1] == 'S'
                && grid[row + 1][col + 1] == 'M'
                && grid[row - 1][col + 1] == 'M' {
                    count += 1;
                }

                if grid[row + 1][col - 1] == 'M'
                && grid[row - 1][col - 1] == 'S'
                && grid[row + 1][col + 1] == 'M'
                && grid[row - 1][col + 1] == 'S' {
                    count += 1;
                }

                if grid[row + 1][col - 1] == 'S'
                && grid[row - 1][col - 1] == 'M'
                && grid[row + 1][col + 1] == 'S'
                && grid[row - 1][col + 1] == 'M' {
                    count += 1;
                }
            }
        }
    }

    println!("part2: {}", count);
}


fn main() {
    part1();
    part2();
}
