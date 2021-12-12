use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;
use std::collections::HashMap;
use std::collections::VecDeque;

type Input = HashMap<String, Vec<String>>;

fn parse_input(filename: &str) -> Input {
    let mut hm:Input = HashMap::new();
    let f = File::open(filename).unwrap();
    for entry in BufReader::new(f).lines().flatten() {
        let spl = entry.split("-").collect::<Vec<&str>>();
        if hm.contains_key(spl[0]) {
            hm.get_mut(spl[0]).unwrap().push(spl[1].to_string());
        } else {
            let mut new_vec = Vec::new();
            new_vec.push(spl[1].to_string());
            hm.insert(spl[0].to_string(), new_vec);
        }
        if hm.contains_key(spl[1]) {
            hm.get_mut(spl[1]).unwrap().push(spl[0].to_string());
        } else {
            let mut new_vec = Vec::new();
            new_vec.push(spl[0].to_string());
            hm.insert(spl[1].to_string(), new_vec);
        }
    }

    hm
}

fn is_big(name: &str) -> bool {
    name.chars().next().unwrap().is_uppercase()
}

fn part1(graph: &Input) -> usize {
    let mut cur_paths: VecDeque<Vec<String>> = VecDeque::new();
    let mut routes = 0;
    cur_paths.push_back(vec!["start".to_string()]);
    while cur_paths.len() != 0 {
        let path = cur_paths.pop_front().unwrap();
        let cur_room: &str = &path[path.len()-1];
        if cur_room == "end" {
            routes += 1;
            continue;
        }
        for connection in &graph[cur_room] {
            if is_big(connection) || !path.contains(connection) {
                let mut new_path = path.clone();
                new_path.push(connection.clone());
                cur_paths.push_back(new_path);
            }
        }
    }

    routes
}

fn part2(graph: &Input) -> usize {
    let mut cur_paths: VecDeque<(bool, Vec<String>)> = VecDeque::new();
    let mut routes = 0;
    cur_paths.push_back((false, vec!["start".to_string()]));
    while cur_paths.len() != 0 {
        let (doubled, path) = cur_paths.pop_front().unwrap();
        let cur_room: &str = &path[path.len()-1];
        if cur_room == "end" {
            routes += 1;
            continue;
        }
        for connection in &graph[cur_room] {
            if is_big(connection) || !path.contains(connection) {
                let mut new_path = path.clone();
                new_path.push(connection.clone());
                cur_paths.push_back((doubled, new_path));
            } else if connection != "start" && !doubled && path.contains(connection) {
                let mut new_path = path.clone();
                new_path.push(connection.clone());
                cur_paths.push_back((true, new_path));
            }
        }
    }

    routes
}

pub fn main() {
    let graph = parse_input("./input/day12/input.txt");

    let p1_timer = Instant::now();
    let p1_result = part1(&graph);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(&graph);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time);  
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day12_test1() {
        let graph = parse_input("./input/day12/test1.txt");
        assert_eq!(part1(&graph), 10);
	}

    #[test]
    fn day12_test2() {
        let graph = parse_input("./input/day12/test1.txt");
        assert_eq!(part2(&graph), 36);
	}

    #[test]
    fn day12_test3() {
        let graph = parse_input("./input/day12/test2.txt");
        assert_eq!(part1(&graph), 19);
	}

    #[test]
    fn day12_test4() {
        let graph = parse_input("./input/day12/test2.txt");
        assert_eq!(part2(&graph), 103);
	}

    #[test]
    fn day12_test5() {
        let graph = parse_input("./input/day12/test3.txt");
        assert_eq!(part1(&graph), 226);
	}

    #[test]
    fn day12_test6() {
        let graph = parse_input("./input/day12/test3.txt");
        assert_eq!(part2(&graph), 3509);
	}
}
