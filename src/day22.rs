use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;
use std::collections::HashSet;

#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
struct Instr {
    turn_on: bool,
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
    z1: i64,
    z2: i64
}

type Input = Vec<Instr>;

fn parse_line(cmd: &str) -> Instr {
    let spl = cmd.split_ascii_whitespace().collect::<Vec<&str>>();
    let pts = spl[1].split(",").collect::<Vec<&str>>();
    let x_spl = pts[0][2..].split("..").collect::<Vec<&str>>();
    let y_spl = pts[1][2..].split("..").collect::<Vec<&str>>();
    let z_spl = pts[2][2..].split("..").collect::<Vec<&str>>();
    Instr {
        turn_on: spl[0] == "on",
        x1: x_spl[0].parse().unwrap(),
        x2: x_spl[1].parse().unwrap(),
        y1: y_spl[0].parse().unwrap(),
        y2: y_spl[1].parse().unwrap(),
        z1: z_spl[0].parse().unwrap(),
        z2: z_spl[1].parse().unwrap()
    }
}

fn parse_input(filename: &str) -> Input {
    let f = File::open(filename).unwrap();
    BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn calc_area(instructions: &Input, x_pos: i64) -> i64 {
    let my_set = instructions.iter().filter(|i| x_pos >= i.x1 && x_pos <= i.x2).collect::<Vec<&Instr>>();
    let mut y_hs = HashSet::new();
    for i in &my_set {
        y_hs.insert(i.y1);
        y_hs.insert(i.y2 + 1);
    }    
    let mut y_pts = y_hs.iter().map(|v| *v).collect::<Vec<i64>>();
    y_pts.sort();
    let mut total_area = 0;
    let mut cur_area = 0;
    let mut first_point = true;
    let mut prev_point = 0;
    for y in y_pts {
        if !first_point {
            total_area += cur_area * (y - prev_point);
        }
        first_point = false;
        prev_point = y;
        let mut v = vec![false; 200000];
        for i in &my_set {
            if y >= i.y1 && y <= i.y2 {
                for j in i.z1..=i.z2 {
                    v[(j+100000) as usize] = i.turn_on;
                }
            }
        } 
        cur_area = v.iter().filter(|v| **v).count() as i64;       
    }
    total_area
}

fn calculate(instructions: &Input, filter_input: bool) -> usize {
    let my_set = if filter_input { instructions.iter().filter(|i| i.x1 >= -50 && i.x2 <= 50 && i.y1 >= -50 && i.y2 <= 50 && i.z1 >= -50 && i.z2 <= 50)
                                   .map(|v| v.clone()).collect::<Vec<Instr>>() } else { instructions.clone() };
    let mut x_hs = HashSet::new();
    for i in &my_set {
        x_hs.insert(i.x1);
        x_hs.insert(i.x2 + 1);
    }

    let mut x_pts = x_hs.iter().map(|v| *v).collect::<Vec<i64>>();
    x_pts.sort();
    let mut total_volume = 0;
    let mut cur_area = 0;
    let mut first_point = true;
    let mut prev_point = 0;
    for x in x_pts {
        if !first_point {
            total_volume += cur_area * (x - prev_point);
        }
        first_point = false;
        prev_point = x;

        cur_area = calc_area(&my_set, x);
    }
    
    total_volume as usize
}

pub fn main() {
    let instructions = parse_input("./input/day22/input.txt");

    let p1_timer = Instant::now();
    let p1_result = calculate(&instructions, true);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = calculate(&instructions, false);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time); 
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day22_test1() {
        let instructions = parse_input("./input/day22/test.txt");
        assert_eq!(calculate(&instructions, true), 590784);
	}
}
