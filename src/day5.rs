use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;
use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
struct Line {
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64
}

type Input = Vec<Line>;

fn parse_line(cmd: &str) -> Line {
    let spl = cmd.split(" -> ").collect::<Vec<&str>>();
    let p1 = spl[0].split(",").collect::<Vec<&str>>();
    let p2 = spl[1].split(",").collect::<Vec<&str>>();

    Line {
        x1: p1[0].parse().unwrap(),
        y1: p1[1].parse().unwrap(),
        x2: p2[0].parse().unwrap(),
        y2: p2[1].parse().unwrap()
    }
}

fn parse_input(filename: &str) -> Input {
    let f = File::open(filename).unwrap();
    BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn part1(lines: &Vec<Line>) -> usize {
    calc(&lines.clone().iter().filter(|l| l.x1 == l.x2 || l.y1 == l.y2).map(|v| *v).collect::<Vec<Line>>())
}

fn part2(lines: &Vec<Line>) -> usize {
    calc(&lines)
}

fn calc(lines: &Vec<Line>) -> usize {
    let mut points: HashMap<(i64, i64), usize> = HashMap::new();

    for line in lines {
        let x_step = match line.x1.cmp(&line.x2) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            Ordering::Equal => 0
        };
        let y_step = match line.y1.cmp(&line.y2) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            Ordering::Equal => 0
        };

        let mut x = line.x1;
        let mut y = line.y1;
        loop {
            if !points.contains_key(&(x, y)) {
                points.insert((x, y), 1);
            } else {
                *points.get_mut(&(x, y)).unwrap()+=1;
            }      
            if x == line.x2 && y == line.y2 {
                break;
            }    
            x += x_step;
            y += y_step;  
        }
    }

    points.iter().filter(|(_, &v)| v > 1).count()
}

pub fn main() {
    let lines = parse_input("./input/day5/input.txt");

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
    fn day5_test1() {
        let lines = parse_input("./input/day5/test.txt");
        assert_eq!(part1(&lines), 5);
	}

    #[test]
    fn day5_test2() {
        let lines = parse_input("./input/day5/test.txt");
        assert_eq!(part2(&lines), 12);
	}

}
