use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;
use std::char;

type Input = Vec<String>;

fn parse_input(filename: &str) -> Input {
    let f = File::open(filename).unwrap();
    BufReader::new(f).lines().flatten().collect()
}

fn reduce(number: &Vec<i64>) -> Vec<i64> {
    let mut depth = 0;
    let mut explode_pos = 0;
    for (i,c) in number.iter().enumerate() {
        if *c == -2 {
            depth += 1;
        } else if *c == -1 {
            depth -= 1;
        }
        if depth > 4 {
            explode_pos = i;
            break;
        }
    }
    if explode_pos != 0 {
        let mut work_number = number.clone();
        let mut left_pos = explode_pos;
        while left_pos > 0 {
            left_pos -= 1;
            if work_number[left_pos] >= 0 {
                break;
            }
        }
        let mut right_pos = explode_pos + 5;
        while right_pos < work_number.len() - 1 {
            right_pos += 1;
            if work_number[right_pos] >= 0 {
                break;
            }
        }
        if left_pos != 0 {
            work_number[left_pos] += work_number[explode_pos + 1];
        }
        if right_pos != number.len() - 1 {
            work_number[right_pos] += work_number[explode_pos + 3]; 
        }
        work_number[explode_pos] = 0;
        return work_number.iter().take(explode_pos+1).chain(work_number.iter().skip(explode_pos + 5)).map(|v| *v).collect();
    }
    let mut split_spot = 0;
    for (i,c) in number.iter().enumerate() {
        if *c > 9 {
            split_spot = i;
            break;
        } 
    }
    if split_spot != 0 {
        let val = number[split_spot];
        let new_vec = vec![-2, val / 2, -3, (val / 2) + (val % 2), -1];
        return number.iter().take(split_spot).chain(new_vec.iter())
                     .chain(number.iter().skip(split_spot + 1)).map(|v| *v).collect();
    }
    Vec::new()
}

fn calc_mag(number: &Vec<char>, pos: usize) -> usize {
    if number[pos].is_numeric() {
        return number[pos].to_digit(10).unwrap() as usize;
    }
    let left_pos = pos + 1;
    let mut right_pos = left_pos;
    let mut depth = 0;
    while !(depth == 0 && number[right_pos] == ',') {
        if number[right_pos] == '[' {
            depth += 1;
        } else if number[right_pos] == ']' {
            depth -= 1;
        }
        right_pos += 1;
    }
    right_pos += 1;
    (3 * calc_mag(number, left_pos)) + (2 * calc_mag(number, right_pos))
    
}

fn part1(numbers: &Vec<String>) -> usize {
    let mut work_str = numbers[0].clone();
    for n in numbers.iter().skip(1) {
        work_str = format!("[{},{}]", work_str, n);
        let mut work_vec:Vec<i64> = work_str.chars()
            .map(|c| match c { '[' => -2, ']' => -1, ',' => -3, _ => c.to_digit(10).unwrap() as i64}).collect();
        loop {
            let new_vec = reduce(&work_vec);   
            if new_vec == Vec::new() {
                break;
            }
            work_vec = new_vec;                
        }
        work_str = work_vec.iter().map(|v| match v { -2 => '[', -1 => ']', -3 => ',', _ => char::from_digit(*v as u32, 10).unwrap()}).collect();
    }
    let final_vec = work_str.chars().collect::<Vec<char>>();
    calc_mag(&final_vec, 0)
}

fn part2(numbers: &Vec<String>) -> usize {
    let mut max_mag = 0;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i == j {
                continue;
            }
            let work_str = format!("[{},{}]", numbers[i], numbers[j]);
            let mut work_vec:Vec<i64> = work_str.chars()
                .map(|c| match c { '[' => -2, ']' => -1, ',' => -3, _ => c.to_digit(10).unwrap() as i64}).collect();
            loop {
                let new_vec = reduce(&work_vec);   
                if new_vec == Vec::new() {
                    break;
                }
                work_vec = new_vec;                
            }
            let final_str:String = work_vec.iter().map(|v| match v { -2 => '[', -1 => ']', -3 => ',', _ => char::from_digit(*v as u32, 10).unwrap()}).collect();
            let final_vec = final_str.chars().collect::<Vec<char>>();
            max_mag = std::cmp::max(max_mag, calc_mag(&final_vec, 0));
        }
    }

    max_mag
}

pub fn main() {
    let strings = parse_input("./input/day18/input.txt");

    let p1_timer = Instant::now();
    let p1_result = part1(&strings);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(&strings);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day16_test1() {
        let binary = parse_input("./input/day16/test1.txt");
        assert_eq!(part1(&binary), 6);
	}

    #[test]
    fn day16_test2() {
        let binary = parse_input("./input/day16/test4.txt");
        assert_eq!(part1(&binary), 16);
	}

    #[test]
    fn day16_test3() {
        let binary = parse_input("./input/day16/test5.txt");
        assert_eq!(part1(&binary), 12);
	}

    #[test]
    fn day16_test4() {
        let binary = parse_input("./input/day16/test6.txt");
        assert_eq!(part1(&binary), 23);
	}

    #[test]
    fn day16_test5() {
        let binary = parse_input("./input/day16/test7.txt");
        assert_eq!(part1(&binary), 31);
	}

    #[test]
    fn day16_test6() {
        let binary = parse_input("./input/day16/test8.txt");
        assert_eq!(part2(&binary), 3);
	}

    #[test]
    fn day16_test7() {
        let binary = parse_input("./input/day16/test9.txt");
        assert_eq!(part2(&binary), 1);
	}
}
