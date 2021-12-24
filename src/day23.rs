use std::time::Instant;
use std::collections::HashSet;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Clone, Eq, PartialEq, Hash)]
#[derive(Debug)]
struct Stack {
    row: usize,
    home: char,
    spots: Vec<char>
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    hall: Vec<char>,
    stacks: Vec<Stack>
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.hall.cmp(&other.hall))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_stacks_p1() -> Vec<Stack> {
    vec![Stack { row: 2, home: 'A', spots: vec!['C', 'C'] }, 
         Stack { row: 4, home: 'B', spots: vec!['A', 'A'] }, 
         Stack { row: 6, home: 'C', spots: vec!['B', 'D'] },
         Stack { row: 8, home: 'D', spots: vec!['D', 'B'] }]
}

fn get_stacks_p2() -> Vec<Stack> {
    vec![Stack { row: 2, home: 'A', spots: vec!['C', 'D', 'D', 'C'] }, 
         Stack { row: 4, home: 'B', spots: vec!['A', 'C', 'B', 'A'] }, 
         Stack { row: 6, home: 'C', spots: vec!['B', 'B', 'A', 'D'] },
         Stack { row: 8, home: 'D', spots: vec!['D', 'A', 'C', 'B'] }]
}

fn stack_is_available(stack: &Stack) -> bool {
    stack.spots.iter().filter(|&&v| v != '.' && v != stack.home).count() == 0
}

fn stack_is_complete(stack: &Stack) -> bool {
    stack.spots.iter().filter(|&&v| v != stack.home).count() == 0
}

fn move_off_stack(stacks: &mut Vec<Stack>, stack_num: usize) -> (usize, char) {
    let stack_len = stacks[stack_num].spots.len();
    for i in 0..stack_len {
        if stacks[stack_num].spots[i] != '.' {
            let name = stacks[stack_num].spots[i];
            stacks[stack_num].spots[i] = '.';
            return (i + 1, name);
        }
    }
    (0, '.')
}

fn send_home(stack: &mut Stack) -> usize {
    for i in 0..stack.spots.len() {
        if i == stack.spots.len() - 1 || stack.spots[i+1] != '.' {
            stack.spots[i] = stack.home;
            return i + 1;
        }
    }
    0
}

fn open_path(hall: &Vec<char>, start: usize, stop: usize) -> bool {
    for i in std::cmp::min(start, stop)..=std::cmp::max(start, stop) {
        if hall[i] != '.' {
            return false;
        }
    }
    true
}

fn cost_per_unit(name: char) -> usize {
    match name {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        _ => 1000
    }
}

fn calc(stacks: Vec<Stack>) -> usize {
    let valid_hall_spots = vec![0, 1, 3, 5, 7, 9, 10];
    let hall = vec!['.'; 11];

    let mut visited = HashSet::new();
    let mut heap = BinaryHeap::new();
    heap.push(State { cost: 0, stacks: stacks, hall: hall } );
    while let Some(State { cost, hall, stacks }) = heap.pop() {
        if visited.contains(&(hall.clone(), stacks.clone())) {
            continue;
        }
        visited.insert((hall.clone(), stacks.clone()));
        
        if stacks.iter().all(|s| stack_is_complete(s)) {
            return cost;
        }

        for i in 0..stacks.len() {
            if !stack_is_available(&stacks[i]) {
                let mut new_stacks = stacks.clone();
                let (dist, name) = move_off_stack(&mut new_stacks, i);
                for j in &valid_hall_spots {
                    if open_path(&hall, stacks[i].row, *j) {
                        let total_dist = dist as i32 + (stacks[i].row as i32 - *j as i32).abs();
                        let mut new_hall = hall.clone();
                        new_hall[*j] = name;
                        heap.push(State { cost: cost + total_dist as usize * cost_per_unit(name),
                                          stacks: new_stacks.clone(), hall: new_hall } );
                    }
                }
            }
        }
        for i in 0..hall.len() {
            if hall[i] != '.' {
                let name = hall[i];
                let mut new_hall = hall.clone();
                new_hall[i] = '.';
                let target_stack = match name { 'A' => 0, 'B' => 1, 'C' => 2, _ => 3 };
                if stack_is_available(&stacks[target_stack]) && open_path(&new_hall, i, stacks[target_stack].row) {
                    let mut new_stacks = stacks.clone();
                    let dist = send_home(&mut new_stacks[target_stack]) + (i as i32 - stacks[target_stack].row as i32).abs() as usize;
                    heap.push(State { cost: cost + dist as usize * cost_per_unit(name),
                        stacks: new_stacks.clone(), hall: new_hall } );
                }
            }
        }
    } 
    0
}

pub fn main() {
    let p1_timer = Instant::now();
    let p1_result = calc(get_stacks_p1());
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time); 

    let p2_timer = Instant::now();
    let p2_result = calc(get_stacks_p2());
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time); 
}

#[cfg(test)]
mod tests {

    use super::*;
    
    fn get_test_stacks_p1() -> Vec<Stack> {
        vec![Stack { row: 2, home: 'A', spots: vec!['B', 'A'] }, 
            Stack { row: 4, home: 'B', spots: vec!['C', 'D'] }, 
            Stack { row: 6, home: 'C', spots: vec!['B', 'C'] },
            Stack { row: 8, home: 'D', spots: vec!['D', 'A'] }]
    }

    #[test]
    fn day23_test1() {
        assert_eq!(calc(get_test_stacks_p1()), 12521);
	}
   
    fn get_test_stacks_p2() -> Vec<Stack> {
        vec![Stack { row: 2, home: 'A', spots: vec!['B', 'D', 'D', 'A'] }, 
            Stack { row: 4, home: 'B', spots: vec!['C', 'C', 'B', 'D'] }, 
            Stack { row: 6, home: 'C', spots: vec!['B', 'B', 'A', 'C'] },
            Stack { row: 8, home: 'D', spots: vec!['D', 'A', 'C', 'A'] }]
    }

    #[test]
    fn day23_test2() {
        assert_eq!(calc(get_test_stacks_p2()), 44169);
	}
}
