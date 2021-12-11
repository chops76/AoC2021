use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;

type Input = Vec<Vec<u32>>;

fn parse_line(line: &str) -> Vec<u32> {
    line.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn parse_input(filename: &str) -> Input {
    let f = File::open(filename).unwrap();
    BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn print_grid(g: &Input) {
    println!("\n\n");
    for i in 0..g.len() {
        for j in 0..g[0].len() {
            print!("{}", g[i][j]);
        }
        println!("");
    }
}

fn do_flash(grid: &mut Input, flash_x: usize, flash_y: usize) {
    grid[flash_y][flash_x] = 11;
    if flash_x > 0 && grid[flash_y][flash_x - 1] <= 9 {
        grid[flash_y][flash_x - 1] += 1;
    }
    if flash_y > 0 && grid[flash_y - 1][flash_x] <= 9 {
        grid[flash_y - 1][flash_x] += 1;
    }
    if flash_x <= grid[0].len() - 2 && grid[flash_y][flash_x + 1] <= 9 {
        grid[flash_y][flash_x + 1] += 1;
    }
    if flash_y <= grid.len() - 2 && grid[flash_y + 1][flash_x] <= 9 {
        grid[flash_y + 1][flash_x] += 1;
    }
    if flash_x > 0 && flash_y > 0 && grid[flash_y - 1][flash_x - 1] <= 9 {
        grid[flash_y - 1][flash_x - 1] += 1;
    }
    if flash_x > 0 && flash_y <= grid.len() - 2 && grid[flash_y + 1][flash_x - 1] <= 9 {
        grid[flash_y + 1][flash_x - 1] += 1;
    }
    if flash_x <= grid[0].len() - 2 && flash_y > 0 && grid[flash_y - 1][flash_x + 1] <= 9 {
        grid[flash_y - 1][flash_x + 1] += 1;
    }
    if flash_x <= grid[0].len() - 2 && flash_y <= grid.len() - 2 && grid[flash_y + 1][flash_x + 1] <= 9 {
        grid[flash_y + 1][flash_x + 1] += 1;
    }
}

fn part1(g: &Input) -> usize {
    let mut grid = g.clone();
    let mut flash_count = 0;
    for _ in 0..100 {
        for i in 0..grid[0].len() {
            for j in 0..grid.len() {
                grid[i][j] += 1;
            }
        }

        while grid.iter().flatten().filter(|&v| *v == 10).count() != 0 {
            let to_flash = grid.iter().flatten().enumerate().filter(|(_, &v)| v == 10).map(|(i, _)| i).collect::<Vec<usize>>();
            for flash in to_flash {
                let flash_x = flash % grid[0].len();
                let flash_y = flash / grid.len();
                do_flash(&mut grid, flash_x, flash_y);
            }
        }

        for i in 0..grid[0].len() {
            for j in 0..grid.len() {
                if grid[i][j] == 11 {
                    grid[i][j] = 0;
                    flash_count += 1;
                }
            }
        }

    }

    flash_count
}

fn part2(g: &Input) -> usize {
    let mut grid = g.clone();
    let mut cycles = 0;
    while grid.iter().flatten().filter(|&v| *v != 0).count() != 0 {
        cycles += 1;
        for i in 0..grid[0].len() {
            for j in 0..grid.len() {
                grid[i][j] += 1;
            }
        }

        while grid.iter().flatten().filter(|&v| *v == 10).count() != 0 {
            let to_flash = grid.iter().flatten().enumerate().filter(|(_, &v)| v == 10).map(|(i, _)| i).collect::<Vec<usize>>();
            for flash in to_flash {
                let flash_x = flash % grid[0].len();
                let flash_y = flash / grid.len();
                do_flash(&mut grid, flash_x, flash_y);
            }
        }

        for i in 0..grid[0].len() {
            for j in 0..grid.len() {
                if grid[i][j] == 11 {
                    grid[i][j] = 0;
                }
            }
        }

    }

    cycles
}

pub fn main() {
    let grid = parse_input("./input/day11/input.txt");
    println!("{:?}", grid);

    let p1_timer = Instant::now();
    let p1_result = part1(&grid);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(&grid);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time);  
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day11_test1() {
        let grid = parse_input("./input/day11/test.txt");
        assert_eq!(part1(&grid), 1656);
	}

    #[test]
    fn day11_test2() {
        let grid = parse_input("./input/day11/test.txt");
        assert_eq!(part2(&grid), 195);
	}
}
