use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;
use std::collections::HashSet;

#[derive(Clone)]
#[derive(Debug)]
struct Display {
    patterns: Vec<String>,
    outputs: Vec<String>
}

type Input = Vec<Display>;

fn parse_line(cmd: &str) -> Display {
    let spl = cmd.split(" | ").collect::<Vec<&str>>();
    let p1 = spl[0].split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
    let p2 = spl[1].split(" ").map(|s| s.to_string()).collect::<Vec<String>>();

    Display {
        patterns: p1,
        outputs: p2
    }
}

fn parse_input(filename: &str) -> Input {
    let f = File::open(filename).unwrap();
    BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn part1(displays: &Input) -> usize {
    displays.iter().map(|d| d.outputs.iter().filter(|o| o.len() == 2 || o.len() == 3 || o.len() == 4 || o.len() == 7).count()).sum()
}

fn part2(displays: &Input) -> usize {
    let mut total = 0;

    for display in displays {
        let one = display.patterns.iter().filter(|p| p.len() == 2).next().unwrap().chars().collect::<HashSet<char>>();
        let seven = display.patterns.iter().filter(|p| p.len() == 3).next().unwrap().chars().collect::<HashSet<char>>();
        let four = display.patterns.iter().filter(|p| p.len() == 4).next().unwrap().chars().collect::<HashSet<char>>();
        let eight = display.patterns.iter().filter(|p| p.len() == 7).next().unwrap().chars().collect::<HashSet<char>>();

        let contains_six = display.patterns.iter().filter(|p| p.len() == 6).collect::<Vec<&String>>();
        let mut nine = HashSet::new();
        for test in &contains_six {
            let test_set = test.chars().collect::<HashSet<char>>();
            if test_set.is_superset(&four) {
                nine = test_set;
                break;
            }
        }
        let mut zero = HashSet::new();
        for test in &contains_six {
            let test_set = test.chars().collect::<HashSet<char>>();
            if test_set != nine && test_set.is_superset(&one) {
                zero = test_set;
                break;
            }
        }
        let mut six = HashSet::new();
        for test in &contains_six {
            let test_set = test.chars().collect::<HashSet<char>>();
            if test_set != nine && test_set != zero {
                six = test_set;
                break;
            }
        }

        let contains_five = display.patterns.iter().filter(|p| p.len() == 5).collect::<Vec<&String>>();
        let mut three = HashSet::new();
        for test in &contains_five {
            let test_set = test.chars().collect::<HashSet<char>>();
            if test_set.is_superset(&one) {
                three = test_set;
                break;
            }
        }      
        let segs = four.difference(&one).map(|&c| c).collect::<HashSet<char>>();
        let mut five = HashSet::new();
        for test in &contains_five {
            let test_set = test.chars().collect::<HashSet<char>>();
            if test_set != three && test_set.is_superset(&segs) {
                five = test_set;
                break;
            }
        }   
        let mut two = HashSet::new();
        for test in &contains_five {
            let test_set = test.chars().collect::<HashSet<char>>();
            if test_set != three && test_set != five {
                two = test_set;
                break;
            }
        }  

        let mut val = 0;
        for output in &display.outputs {
            val *= 10;
            let segs = output.chars().collect::<HashSet<char>>();
            if segs == zero { val += 0 };
            if segs == one { val += 1 };
            if segs == two { val += 2 };
            if segs == three { val += 3 };
            if segs == four { val += 4 };
            if segs == five { val += 5 };
            if segs == six { val += 6 };
            if segs == seven { val += 7 };
            if segs == eight { val += 8 };
            if segs == nine { val += 9 };
        }
        total += val;
    }
    
    total
}

pub fn main() {
    let displays = parse_input("./input/day8/input.txt");

    let p1_timer = Instant::now();
    let p1_result = part1(&displays);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(&displays);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day8_test1() {
        let displays = parse_input("./input/day8/test1.txt");
        assert_eq!(part1(&displays), 0);
	}

    #[test]
    fn day8_test2() {
        let displays = parse_input("./input/day8/test2.txt");
        assert_eq!(part1(&displays), 26);
	}

    #[test]
    fn day8_test3() {
        let displays = parse_input("./input/day8/test1.txt");
        assert_eq!(part2(&displays), 5353);
	}

    #[test]
    fn day8_test4() {
        let displays = parse_input("./input/day8/test2.txt");
        assert_eq!(part2(&displays), 61229);
	}

}
