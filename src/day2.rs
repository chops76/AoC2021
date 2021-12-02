use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;

#[derive(Debug)]
enum Command {
    Forward(i64),
    Up(i64),
    Down(i64)
}

type Input = Vec<Command>;

fn parse_line(cmd: &str) -> Command {
    let spl = cmd.split_ascii_whitespace().collect::<Vec<&str>>();
    let val = spl[1].parse().unwrap();
    match spl[0] {
        "forward" => Command::Forward(val),
        "up" => Command::Up(val),
        "down" => Command::Down(val),
        _ => { println!("Invalid Command"); Command::Forward(val)}
    }
}

fn parse_input(filename: &str) -> Input {
    let f = File::open(filename).unwrap();
    BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn part1(cmds: &Input) -> i64 {
    let mut x = 0;
    let mut y = 0;
    for cmd in cmds {
        match cmd {
            Command::Forward(amt) => { x += amt; },
            Command::Up(amt) => { y -= amt; },
            Command::Down(amt) => { y += amt; }
        }
    }
    x * y
}

fn part2(cmds: &Input) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for cmd in cmds {
        match cmd {
            Command::Forward(amt) => { x += amt; y += aim * amt; },
            Command::Up(amt) => { aim -= amt; },
            Command::Down(amt) => { aim += amt; }
        }
    }
    x * y
}

pub fn main() {
    let cmds = parse_input("./input/day2/input.txt");

    let p1_timer = Instant::now();
    let p1_result = part1(&cmds);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(&cmds);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day1_test1() {
        let cmds = parse_input("./input/day2/test.txt");
        assert_eq!(part1(&cmds), 150);
	}

    #[test]
    fn day1_test2() {
        let cmds = parse_input("./input/day2/test.txt");
        assert_eq!(part2(&cmds), 900);
	}
}
