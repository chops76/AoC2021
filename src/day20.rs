use std::io::Read;
use std::fs::File;
use std::time::Instant;
use std::collections::HashSet;
use std::collections::VecDeque;

type Input = (Vec<bool>, VecDeque<VecDeque<bool>>);

fn parse_input(path: &str) -> Input {
    let mut fstr = String::new();

    File::open(path).unwrap().read_to_string(&mut fstr).unwrap();
    let spl = fstr.split("\n\n").collect::<Vec<&str>>();

    let algo = spl[0].chars().map(|c| c == '#').collect::<Vec<bool>>();
    let grid = spl[1].split("\n").map(|s| s.chars().map(|c| c == '#').collect()).collect();
 
    (algo, grid)
}

fn expand_grid(grid: &mut VecDeque<VecDeque<bool>>, default_val: bool) {
    for i in 0..grid.len() {
        grid[i].push_front(default_val);
        grid[i].push_back(default_val);
    }
    let mut tmp = VecDeque::new();
    tmp.resize(grid[0].len(), default_val);
    grid.push_front(tmp.clone());
    grid.push_back(tmp);
}

fn print_grid(g: &VecDeque<VecDeque<bool>>) {
    println!("\n\n");
    for i in 0..g.len() {
        for j in 0..g[0].len() {
            print!("{}", match g[i][j] { true => '#', false => '.' });
        }
        println!("");
    }
}

fn enhance(algo: &Vec<bool>, work_grid: &VecDeque<VecDeque<bool>>, default_val: bool) -> VecDeque<VecDeque<bool>> {
    let mut new_grid = work_grid.clone();
    let pts:Vec<(i32, i32)> = vec![(-1, -1), (-1, 0), (-1,1), (0,-1), (0,0), (0,1), (1,-1), (1,0), (1,1)];
    for i in 0..work_grid.len() {
        for j in 0..work_grid[i].len() {
            let mut cur_val = 0;
            for p in &pts {
                cur_val *= 2;
                if i as i32 + p.0 < 0 || i as i32 + p.0 > (work_grid.len() as i32 - 1) || 
                   j as i32 + p.1 < 0 || j as i32 + p.1 > (work_grid[0].len() as i32 - 1) {
                    if default_val == true {
                        cur_val += 1;
                    }
                } else if work_grid[(i as i32 + p.0) as usize][(j as i32 + p.1) as usize] {
                    cur_val += 1;
                }
            }
            new_grid[i][j] = algo[cur_val];
        }
    }

    new_grid
}

fn part1(algo: &Vec<bool>, grid: &VecDeque<VecDeque<bool>>, flip_default: bool) -> usize {
    let mut default_val = false;
    let mut work_grid = grid.clone();

    for _ in 0..2 {
        expand_grid(&mut work_grid, default_val);
        work_grid = enhance(algo, &work_grid, default_val);
        if flip_default {
            default_val = !default_val;
        }
    }
    work_grid.iter().flatten().filter(|v| **v).count()
}

fn part2(algo: &Vec<bool>, grid: &VecDeque<VecDeque<bool>>, flip_default: bool) -> usize {
    let mut default_val = false;
    let mut work_grid = grid.clone();

    for _ in 0..50 {
        expand_grid(&mut work_grid, default_val);
        work_grid = enhance(algo, &work_grid, default_val);
        if flip_default {
            default_val = !default_val;
        }
    }
    work_grid.iter().flatten().filter(|v| **v).count()
}

pub fn main() {
    let (algo, grid) = parse_input("./input/day20/input.txt");

    let p1_timer = Instant::now();
    let p1_result = part1(&algo, &grid, true);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(&algo, &grid, true);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time); 
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day20_test1() {
        let (algo, grid) = parse_input("./input/day20/test.txt");
        assert_eq!(part1(&algo, &grid, false), 35);
	}

    #[test]
    fn day20_test2() {
        let (algo, grid) = parse_input("./input/day20/test.txt");
        assert_eq!(part2(&algo, &grid, false), 3351);
	}
}
