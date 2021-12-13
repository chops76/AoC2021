use std::io::Read;
use std::fs::File;
use std::time::Instant;
use std::collections::HashSet;

type Input = (HashSet<(usize, usize)>, Vec<(bool, usize)>);

fn parse_input(path: &str) -> Input {
    let mut fstr = String::new();

    File::open(path).unwrap().read_to_string(&mut fstr).unwrap();
    let spl = fstr.split("\n\n").collect::<Vec<&str>>();

    let mut points = HashSet::new();
    for pt_str in spl[0].split("\n") {
        let pt_spl = pt_str.split(",").collect::<Vec<&str>>();
        points.insert((pt_spl[0].parse().unwrap(), pt_spl[1].parse().unwrap()));
    }

    let mut folds = Vec::new();
    for fold_str in spl[1].split("\n") {
        let word_spl = fold_str.split("along ").collect::<Vec<&str>>();
        let inst_spl = word_spl[1].split("=").collect::<Vec<&str>>();
        folds.push((inst_spl[0] == "x", inst_spl[1].parse().unwrap()));
    }
    
    (points, folds)
}

fn do_fold(points: &HashSet<(usize, usize)>, coord: usize, is_x_fold: bool) -> HashSet<(usize, usize)>
{
    let mut new_points = HashSet::new();
    for point in points {
        let mut new_x = point.0;
        let mut new_y = point.1;
        if is_x_fold && new_x > coord {
            new_x = coord - (new_x - coord);
        } else if !is_x_fold && new_y > coord {
            new_y = coord - (new_y - coord);
        }
        new_points.insert((new_x, new_y));
    }
    
    new_points
}

fn part1(points: &HashSet<(usize, usize)>, folds: &Vec<(bool, usize)>) -> usize {
    let (is_x_fold, coord) = folds[0];
    do_fold(points, coord, is_x_fold).len()
}

fn part2(points: &HashSet<(usize, usize)>, folds: &Vec<(bool, usize)>) -> usize {
    let mut cur_points = points.clone();
    
    for fold in folds {
        let (is_x_fold, coord) = *fold;
        cur_points = do_fold(&cur_points, coord, is_x_fold);
    }

    let max_x = cur_points.iter().map(|(x,_)| x).max().unwrap();
    let max_y = cur_points.iter().map(|(_,y)| y).max().unwrap();
    for y in 0..=*max_y {
        for x in 0..=*max_x {
            if cur_points.contains(&(x, y)) {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!("");
    }

    cur_points.len()
}

pub fn main() {
    let (points, folds) = parse_input("./input/day13/input.txt");

    let p1_timer = Instant::now();
    let p1_result = part1(&points, &folds);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(&points, &folds);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day13_test1() {
        let (points, folds) = parse_input("./input/day13/test.txt");
        assert_eq!(part1(&points, &folds), 17);
	}

    #[test]
    fn day13_test2() {
        let (points, folds) = parse_input("./input/day13/test.txt");
        assert_eq!(part2(&points, &folds), 16);
	}
}
