use std::io::Read;
use std::fs::File;
use std::time::Instant;

type Input = Vec<i64>;

fn parse_input(path: &str) -> Input {
    let mut fstr = String::new();

    File::open(path).unwrap().read_to_string(&mut fstr).unwrap();
    fstr.split(",").map(|s| s.parse().unwrap()).collect()
}

fn part1(p: &Vec<i64>) -> i64 {
    let mut positions = p.clone();
    positions.sort();
    let mid = positions[positions.len() / 2];
    positions.iter().map(|v| (v - mid).abs()).sum()
}

fn cost(dist: i64) -> i64 {
    dist * (dist + 1) / 2
}

fn part2(p: &Vec<i64>) -> i64 {
    let mut positions = p.clone();
    positions.sort();
    let mut best_amt = i64::MAX;
    for i in *positions.first().unwrap()..=*positions.last().unwrap() {
        let val:i64 = positions.iter().map(|v| cost((v - i).abs())).sum();
        best_amt = i64::min(best_amt, val);
    }

    best_amt
}

pub fn main() {
    let positions = parse_input("./input/day7/input.txt");

    let p1_timer = Instant::now();
    let p1_result = part1(&positions);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(&positions);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time); 
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day7_test1() {
        let positions = parse_input("./input/day7/test.txt");
        assert_eq!(part1(&positions), 37);
	}

    #[test]
    fn day7_test2() {
        let positions = parse_input("./input/day7/test.txt");
        assert_eq!(part2(&positions), 168);
	}

}
