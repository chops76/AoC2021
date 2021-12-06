use std::io::Read;
use std::fs::File;
use std::time::Instant;

type Input = Vec<i64>;

fn parse_input(path: &str) -> Input {
    let mut fstr = String::new();

    File::open(path).unwrap().read_to_string(&mut fstr).unwrap();
    fstr.split(",").map(|s| s.parse().unwrap()).collect()
}


fn part1(a: &Vec<i64>) -> usize {
    let mut ages = a.clone();
    for _ in 0..80 {
        let cur_len = ages.len();
        for i in 0..cur_len {
            if ages[i] == 0 {
                ages[i] = 6;
                ages.push(8);
            } else {
                ages[i] -= 1;
            }
        }
    }
    ages.len()
}

fn part2(a: &Vec<i64>) -> usize {
    let mut fish = vec![0;9];
    for age in a {
        fish[*age as usize] += 1;
    }

    for _ in 0..256 {
        let spawning = fish[0];
        for i in 0..8 {
            fish[i] = fish[i+1];
        }
        fish[6] += spawning;
        fish[8] = spawning;
    }

    fish.iter().sum()
}

pub fn main() {
    let ages = parse_input("./input/day6/input.txt");

    let p1_timer = Instant::now();
    let p1_result = part1(&ages);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(&ages);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time); 
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day6_test1() {
        let ages = parse_input("./input/day6/test.txt");
        assert_eq!(part1(&ages), 5934);
	}

    #[test]
    fn day6_test2() {
        let ages = parse_input("./input/day6/test.txt");
        assert_eq!(part2(&ages), 26984457539);
	}

}
