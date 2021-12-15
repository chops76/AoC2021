use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;
use std::collections::HashMap;

type Input = Vec<Vec<usize>>;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize,usize)
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_line(line: &str) -> Vec<usize> {
    line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect()
}

fn parse_input(filename: &str) -> Input {
    let f = File::open(filename).unwrap();
    BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn find_path(grid: &Input) -> usize {
    let mut heap = BinaryHeap::new();

    let mut dist = HashMap::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            dist.insert((j,i), usize::MAX);
        }
    }

    *dist.get_mut(&(0,0)).unwrap() = 0;
    heap.push(State { cost: 0, position: (0,0) });
    let goal = (grid[0].len() - 1, grid.len() - 1);

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return cost;
        }
        if cost > dist[&position] {
            continue;
        }
        
        if position.0 > 0 {
            let new_pos = (position.0 - 1, position.1);
            let next = State { cost: cost + grid[new_pos.1][new_pos.0], position: new_pos };
            if next.cost < dist[&next.position] {
                *dist.get_mut(&next.position).unwrap() = next.cost;
                heap.push(next);
            }
        }
        if position.1 > 0 {
            let new_pos = (position.0, position.1 - 1);
            let next = State { cost: cost + grid[new_pos.1][new_pos.0], position: new_pos };
            if next.cost < dist[&next.position] {
                *dist.get_mut(&next.position).unwrap() = next.cost;
                heap.push(next);
            }
        }
        if position.0 < grid[0].len() - 1 {
            let new_pos = (position.0 + 1, position.1);
            let next = State { cost: cost + grid[new_pos.1][new_pos.0], position: new_pos };
            if next.cost < dist[&next.position] {
                *dist.get_mut(&next.position).unwrap() = next.cost;
                heap.push(next);
            }
        }
        if position.1 < grid.len() - 1 {
            let new_pos = (position.0, position.1 + 1);
            let next = State { cost: cost + grid[new_pos.1][new_pos.0], position: new_pos };
            if next.cost < dist[&next.position] {
                *dist.get_mut(&next.position).unwrap() = next.cost;
                heap.push(next);
            }
        }
    }
    
    dist[&(grid.len() - 1, grid[0].len() - 1)]

}
fn part1(grid: &Input) -> usize {
    find_path(grid)
}

fn part2(grid: &Input) -> usize {
    let mut new_grid = grid.clone();
    for i in 0..grid.len() {
        for j in 0..4 {
            for k in 0..grid[i].len() {
                let mut new_val = grid[i][k] + j + 1;
                if new_val > 9 {
                    new_val -= 9;
                }
                new_grid[i].push(new_val);
            }
        }
    }
    for i in 0..4 {
        for j in 0..grid.len() {
            let mut new_line = new_grid[j].clone();
            for k in 0..new_line.len() {
                new_line[k] += i + 1;
                if new_line[k] > 9 {
                    new_line[k] -= 9;
                }
            }
            new_grid.push(new_line);
        }
    }

    find_path(&new_grid)
}


pub fn main() {
    let grid = parse_input("./input/day15/input.txt");

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
    fn day15_test1() {
        let grid = parse_input("./input/day15/test.txt");
        assert_eq!(part1(&grid), 40);
	}

    #[test]
    fn day15_test2() {
        let grid = parse_input("./input/day15/test.txt");
        assert_eq!(part2(&grid), 315);
	}
}
