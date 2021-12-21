use std::time::Instant;
use std::collections::HashMap;

fn part1(p1_start: usize, p2_start: usize) -> usize {
    let mut p1_pos = p1_start;
    let mut p2_pos = p2_start;
    let mut p1_score = 0;
    let mut p2_score = 0;
    let mut roll_count = 0;

    let mut dice_value = 1;
    loop {
        let mut roll_value = 0;
        for _ in 0..3 {
            roll_value += dice_value;
            dice_value = dice_value + 1;
            if dice_value > 100 {
                dice_value = 1;
            }
        }
        roll_count += 3;
        p1_pos += roll_value;
        while p1_pos > 10 {
            p1_pos -= 10;
        }
        p1_score += p1_pos;
        if p1_score >= 1000 {
            return p2_score * roll_count;
        }

        let mut roll_value = 0;
        for _ in 0..3 {
            roll_value += dice_value;
            dice_value = dice_value + 1;
            if dice_value > 100 {
                dice_value = 1;
            }
        }
        roll_count += 3;
        p2_pos += roll_value;
        while p2_pos > 10 {
            p2_pos -= 10;
        }
        p2_score += p2_pos;
        if p2_score >= 1000 {
            return p1_score * roll_count;
        }
    }
}

#[derive(PartialEq)]
#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
struct State {
    p1_pos: u64,
    p2_pos: u64,
    p1_score: u64,
    p2_score: u64
}

fn part2(p1_start: usize, p2_start: usize) -> u64 {
    let mut hm:HashMap<State, u64> = HashMap::new();
    let combos = vec![(3,1), (4,3), (5,6), (6,7), (7,6), (8,3), (9,1)];
    let mut p1_winners = 0;
    let mut p2_winners = 0;
    hm.insert(State {p1_pos: p1_start as u64, p2_pos: p2_start as u64, p1_score: 0, p2_score: 0}, 1);
    while hm.len() != 0 {
        let mut new_map = HashMap::new();
        for (state, count) in &hm {
            for (val, freq) in &combos {
                let mut new_pos = state.p1_pos + val;
                if new_pos > 10 {
                    new_pos -= 10;
                }
                let new_score = state.p1_score + new_pos;
                if new_score >= 21 {
                    p1_winners += count * freq;
                } else {
                    *new_map.entry(State {p1_pos: new_pos, p2_pos: state.p2_pos, p1_score: new_score, p2_score: state.p2_score})
                            .or_insert(0) += count * freq;
                }
            }
        }

        hm = new_map;
        let mut new_map = HashMap::new();
        for (state, count) in &hm {
            for (val, freq) in &combos {
                let mut new_pos = state.p2_pos + val;
                if new_pos > 10 {
                    new_pos -= 10;
                }
                let new_score = state.p2_score + new_pos;
                if new_score >= 21 {
                    p2_winners += count * freq;
                } else {
                    *new_map.entry(State {p1_pos: state.p1_pos, p2_pos: new_pos, p1_score: state.p1_score, p2_score: new_score})
                            .or_insert(0) += count * freq;
                }
            }
        }  
        hm = new_map;      
    }

    std::cmp::max(p1_winners, p2_winners)
}

pub fn main() {
    let p1_timer = Instant::now();
    let p1_result = part1(6, 4);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(6, 4);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day20_test1() {
        assert_eq!(part1(4, 8), 739785);
	}

    #[test]
    fn day20_test2() {
        assert_eq!(part2(4, 8), 444356092776315);
	}
}
