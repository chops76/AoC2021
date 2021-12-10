use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;
use std::collections::VecDeque;

type Input = Vec<String>;

fn parse_input(filename: &str) -> Input {
    let f = File::open(filename).unwrap();
    BufReader::new(f).lines().flatten().collect()
}

fn score(line: &str) -> usize {
    let mut stack = VecDeque::new();
    for c in line.chars() {
        match c {
            '{' | '(' | '[' | '<' => stack.push_back(c),
            '}' => { if stack.len() == 0 || stack.pop_back().unwrap() != '{' { return 1197 } },
            ')' => { if stack.len() == 0 || stack.pop_back().unwrap() != '(' { return 3 } },
            ']' => { if stack.len() == 0 || stack.pop_back().unwrap() != '[' { return 57 } },
            '>' => { if stack.len() == 0 || stack.pop_back().unwrap() != '<' { return 25137 } },
            _ => println!("Illegal Character")
        }
    }

    0
}

fn part1(lines: &Input) -> usize {
    lines.iter().map(|l| score(l)).sum()
}

fn completion_score(line: &str) -> usize {
    let mut stack = VecDeque::new();
    for c in line.chars() {
        match c {
            '{' | '(' | '[' | '<' => stack.push_back(c),
            '}' | ')' | ']' | '>' => { stack.pop_back(); },
            _ => println!("Illegal Character")
        }
    }
    let mut score = 0;
    while stack.len() != 0 {
        score *= 5;
        score += match stack.pop_back().unwrap() { 
            '(' => 1, '[' => 2, '{' => 3, '<' => 4, _ => {println!("Stack Error"); 0 }
        };
    }
    
    score
}

fn part2(lines: &Input) -> usize {
    let mut scores = lines.iter().filter(|l| score(l) == 0).map(|l| completion_score(l)).collect::<Vec<usize>>();

    scores.sort();
    scores[scores.len() / 2]
}

pub fn main() {
    let lines = parse_input("./input/day10/input.txt");

    let p1_timer = Instant::now();
    let p1_result = part1(&lines);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(&lines);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time);  
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day10_test1() {
        let lines = parse_input("./input/day10/test.txt");
        assert_eq!(part1(&lines), 26397);
	}

    #[test]
    fn day10_test2() {
        let lines = parse_input("./input/day10/test.txt");
        assert_eq!(part2(&lines), 288957);
	}
}
