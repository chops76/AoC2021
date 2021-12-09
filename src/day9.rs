use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;
use std::collections::VecDeque;
use std::collections::HashSet;

type Input = Vec<Vec<u32>>;

fn parse_line(line: &str) -> Vec<u32> {
    line.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn parse_input(filename: &str) -> Input {
    let f = File::open(filename).unwrap();
    BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn low_points(grid: &Input) -> Vec<(usize, usize)>
{
    let mut ret_vec = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if (i == 0 || grid[i][j] < grid[i-1][j]) &&
               (j == 0 || grid[i][j] < grid[i][j-1]) && 
               (i == grid.len() - 1 || grid[i][j] < grid[i+1][j]) &&
               (j == grid[i].len() - 1 || grid[i][j] < grid[i][j+1]) {
                   ret_vec.push((i, j));
            }
        }
    }

    ret_vec
}

fn part1(grid: &Input) -> u32 {
    low_points(grid).iter().map(|p| grid[p.0][p.1] + 1).sum()
}

fn basin_size(grid: &Input, x: usize, y: usize) -> usize
{
    let mut to_check = VecDeque::new();
    let mut basin = HashSet::new();
    to_check.push_back((x, y));
    while to_check.len() != 0 {
        let cur_pt = to_check.pop_front().unwrap();
        let (i,j) = cur_pt;
        if basin.contains(&cur_pt) || grid[i][j] == 9 {
            continue;
        }
        
        basin.insert(cur_pt);

        if i != 0 {
            to_check.push_back((i-1, j));
        }
        if j != 0 {
            to_check.push_back((i, j-1));
        }
        if i < grid.len() - 1 {
            to_check.push_back((i+1, j));
        }
        if j < grid[0].len() - 1 {
            to_check.push_back((i, j+1));
        }
    }

    basin.len()
}

fn part2(grid: &Input) -> usize {
    let points = low_points(grid);

    let mut sizes = points.iter().map(|p| basin_size(grid, p.0, p.1)).collect::<Vec<usize>>();
    sizes.sort();
    sizes.reverse();
    sizes[0] * sizes[1] * sizes[2]
}

pub fn main() {
    let grid = parse_input("./input/day9/input.txt");

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
    fn day9_test1() {
        let grid = parse_input("./input/day9/test.txt");
        assert_eq!(part1(&grid), 15);
	}

    #[test]
    fn day9_test2() {
        let grid = parse_input("./input/day9/test.txt");
        assert_eq!(part2(&grid), 1134);
	}
}
