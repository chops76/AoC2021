use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;

type Input = Vec<String>;

fn parse_input(filename: &str) -> Input {
    let f = File::open(filename).unwrap();
    BufReader::new(f).lines().flatten().collect()
}

fn part1(cmds: &Input) -> usize {
    let mut counts = Vec::new();
    for i in 0..cmds[0].as_bytes().len() {
        counts.push(cmds.iter().filter(|s| s.as_bytes()[i] == '1' as u8).count());
    }
    
    let gamma:usize = counts.iter().enumerate()
                            .map(|(i, n)| if *n > (cmds.len()/2) { 1 << (counts.len()-1-i) } else { 0 })
                            .sum();
    let epsilon:usize = counts.iter().enumerate()
                            .map(|(i, n)| if *n <= (cmds.len()/2) { 1 << (counts.len()-1-i) } else { 0 })
                            .sum();
    gamma * epsilon
}

fn calc_ox(cmds: &Input) -> usize {
    let mut cmd_list = cmds.clone();
    let mut oxygen = 0;

    for i in 0..cmd_list[0].as_bytes().len() {
        let count = cmd_list.iter().filter(|s| s.as_bytes()[i] == '1' as u8).count();
        let filter_for = if count >= cmd_list.len() - count { '1' as u8 } else { '0' as u8 };
        cmd_list = cmd_list.iter().filter(|s| s.as_bytes()[i] == filter_for)
                           .map(|s| s.clone()).collect();
        if cmd_list.len() == 1 {
            oxygen = usize::from_str_radix(&cmd_list[0], 2).unwrap();
            break;
        }
    }   
    
    oxygen
}

fn calc_co2(cmds: &Input) -> usize {
    let mut cmd_list = cmds.clone();
    let mut co2 = 0;

    for i in 0..cmd_list[0].as_bytes().len() {
        let count = cmd_list.iter().filter(|s| s.as_bytes()[i] == '0' as u8).count();
        let filter_for = if count <= cmd_list.len() - count { '0' as u8 } else { '1' as u8 };
        cmd_list = cmd_list.iter().filter(|s| s.as_bytes()[i] == filter_for).map(|s| s.clone()).collect();
        if cmd_list.len() == 1 {
            co2 = usize::from_str_radix(&cmd_list[0], 2).unwrap();
            break;
        }
    }    

    co2
}

fn part2(cmds: &Input) -> usize {
    calc_ox(cmds) * calc_co2(cmds)
}

pub fn main() {
    let nums = parse_input("./input/day3/input.txt");

    let p1_timer = Instant::now();
    let p1_result = part1(&nums);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(&nums);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day1_test1() {
        let cmds = parse_input("./input/day3/test.txt");
        assert_eq!(part1(&cmds), 198);
	}

    #[test]
    fn day1_test2() {
        let cmds = parse_input("./input/day3/test.txt");
        assert_eq!(calc_ox(&cmds), 23);
	}

    #[test]
    fn day1_test3() {
        let cmds = parse_input("./input/day3/test.txt");
        assert_eq!(calc_co2(&cmds), 10);
	}

    #[test]
    fn day1_test4() {
        let cmds = parse_input("./input/day3/test.txt");
        assert_eq!(part2(&cmds), 230);
	}
}
