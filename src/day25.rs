use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;
use std::collections::HashSet;

type Input = Vec<Vec<char>>;

fn parse_line(row: &str) -> Vec<char> {
    row.chars().collect::<Vec<char>>()
}

fn parse_input(filename: &str) -> Input {
    let f = File::open(filename).unwrap();
    BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn part1(orig_grid: &Input) -> usize {
    let mut grid = orig_grid.clone();
    let mut num_steps = 1;
    let x_len = grid.len();
    let y_len = grid[0].len();
    loop {
        let mut right_movers = HashSet::new();
        let mut down_movers = HashSet::new();
        for i in 0..x_len {
            for j in 0..y_len {
                if grid[i][j] == '>' && grid[i][(j+1) % y_len] == '.' {
                    right_movers.insert((i, j));
                } 
            }
        }
        for (x, y) in &right_movers {
            grid[*x][*y] = '.';
            grid[*x][(*y+1) % y_len] = '>';
        }

        for i in 0..x_len {
            for j in 0..y_len {
                if grid[i][j] == 'v' && grid[(i+1) % x_len][j] == '.' {
                    down_movers.insert((i, j));
                }
            }
        }
        for (x, y) in &down_movers {
            grid[*x][*y] = '.';
            grid[(*x + 1) % x_len][*y] = 'v';
        }

        if right_movers.len() == 0 && down_movers.len() == 0 {
            return num_steps;
        }
        num_steps += 1;
    }
}

pub fn main() {
    let grid = parse_input("./input/day25/input.txt");

    let p1_timer = Instant::now();
    let p1_result = part1(&grid);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day25_test1() {
        let grid = parse_input("./input/day25/test.txt");
        assert_eq!(part1(&grid), 58);
	}
}
