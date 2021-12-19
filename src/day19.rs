use std::io::Read;
use std::fs::File;
use std::time::Instant;
use std::collections::HashSet;

#[derive(PartialEq)]
#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64
}

type Input = Vec<Vec<Point>>;

fn parse_input(path: &str) -> Input {
    let mut fstr = String::new();

    File::open(path).unwrap().read_to_string(&mut fstr).unwrap();
    let spl = fstr.split("\n\n").collect::<Vec<&str>>();

    let mut points = Vec::new();
    for scanner in spl {
        let mut scanner_points = Vec::new();
        for rule_str in scanner.split("\n").skip(1) {
            let strs = rule_str.split(",").collect::<Vec<&str>>();
            scanner_points.push(Point { x: strs[0].parse().unwrap(), y: strs[1].parse().unwrap(), z: strs[2].parse().unwrap()});
        }
        points.push(scanner_points);
    }
    
    points
}

fn generate_rotations(points: &Vec<Point>) -> Vec<Vec<Point>> {
    let mut ret_vec = Vec::new();
    let mut work_vec = points.clone();
    for _ in 0..2 {
        for _ in 0..3 {
            work_vec = work_vec.iter().map(|p| roll(p)).collect();
            ret_vec.push(work_vec.clone());
            for _ in 0..3 {
                work_vec = work_vec.iter().map(|p| turn(p)).collect();
                ret_vec.push(work_vec.clone());
            }
        }
        work_vec = work_vec.iter().map(|p| roll(&turn(&roll(p)))).collect();
    }

    ret_vec
}

fn roll(point: &Point) -> Point {
    Point { x: point.x, y: point.z, z: -point.y }
}

fn turn(point: &Point) -> Point {
    Point { x: -point.y, y: point.x, z: point.z }
}

fn matching(p1: &Vec<Point>, p2: &Vec<Point>) -> (bool, i64, i64, i64) {
    for i in 0..p1.len() {
        for j in 0..p2.len() {
            let xd = p1[i].x - p2[j].x;
            let yd = p1[i].y - p2[j].y;
            let zd = p1[i].z - p2[j].z;
            let mut count = 0;
            for p in p2 {
                if p1.contains(&Point { x: p.x + xd, y: p.y + yd, z: p.z + zd } ) {
                    count += 1;
                    if count >= 12 {
                        return (true, xd, yd, zd);
                    }
                }
            }
        }
    }
    (false, 0, 0, 0)
}

fn part1(points: &Input) -> (usize, i64) {
    let scanners = points.iter().map(|p| generate_rotations(p)).collect::<Vec<Vec<Vec<Point>>>>();
    let mut world_points = scanners[0][0].clone();
    let mut matched_scanners = HashSet::new();
    let mut scanner_pos = Vec::new();
    scanner_pos.push(Point { x: 0, y: 0, z: 0 });
    matched_scanners.insert(0);
    while matched_scanners.len() != scanners.len() {
        for i in 0..scanners.len() {
            if matched_scanners.contains(&i) {
                continue;
            }
            for rot in 0..24 {
                let (found, xd, yd, zd) = matching(&world_points, &scanners[i][rot]);
                if found {
                    matched_scanners.insert(i);
                    println!("Matched {}", i);
                    for p in &scanners[i][rot] {
                        let new_p = Point { x: p.x + xd, y: p.y + yd, z: p.z + zd };
                        scanner_pos.push(Point {x: xd, y: yd, z: zd });
                        if !world_points.contains(&new_p) {
                            world_points.push(new_p);
                        }
                    }
                    break;
                }
            }
        }
    }
    
    let mut max_dist = 0;
    for i in 0..scanner_pos.len() {
        for j in 0..scanner_pos.len() {
            if i == j {
                continue;
            }
            max_dist = std::cmp::max(max_dist, (scanner_pos[i].x - scanner_pos[j].x).abs() +
                                               (scanner_pos[i].y - scanner_pos[j].y).abs() +
                                               (scanner_pos[i].z - scanner_pos[j].z).abs());
        }
    }
    (world_points.len(), max_dist)
}

pub fn main() {
    let points = parse_input("./input/day19/input.txt");

    let p1_timer = Instant::now();
    let (p1_result, p2_result) = part1(&points);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 2: {}", p2_result);
    println!("Total Time: {:?}", p1_time);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day19_test1() {
        let points = parse_input("./input/day19/test.txt");
        assert_eq!(part1(&points).0, 79);
	}

    #[test]
    fn day19_test2() {
        let points = parse_input("./input/day19/test.txt");
        assert_eq!(part1(&points).1, 3621);
	}
}
