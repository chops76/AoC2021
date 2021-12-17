use std::time::Instant;

fn check_for_y_hit(vel: i64, y_min: i64, y_max: i64) -> i64 {
    let mut y_pos = 0;
    let mut cur_vel = vel;
    let mut max_height = 0;
    while y_pos >= y_min {
        if y_pos <= y_max {
            return max_height;
        }
        y_pos += cur_vel;
        cur_vel -= 1;
        if cur_vel == 0 {
            max_height = y_pos;
        }
    }
    0
}

fn check_for_hit(x_vel: i64, y_vel: i64, x_min: i64, x_max: i64, y_min: i64, y_max: i64) -> bool {
    let mut y_pos = 0;
    let mut x_pos = 0;
    let mut cur_x_vel = x_vel;
    let mut cur_y_vel = y_vel;
    while y_pos >= y_min && x_pos <= x_max {
        if y_pos <= y_max && x_pos >= x_min {
            return true;
        }
        x_pos += cur_x_vel;
        if cur_x_vel > 0 {
            cur_x_vel -= 1;
        }
        y_pos += cur_y_vel;
        cur_y_vel -= 1;
    }
    false
}

fn part1(y_min: i64, y_max: i64) -> i64 {
    (1..1000).map(|i| check_for_y_hit(i, y_min, y_max)).max().unwrap()
}

fn part2(x_min: i64, x_max: i64, y_min: i64, y_max: i64) -> usize {
    let highest_y_val = (0..1000).map(|i| (i, check_for_y_hit(i, y_min, y_max))).max_by_key(|(_, i)| *i).unwrap().0;
    let mut count = 0;
    for y_vel in y_min..=highest_y_val {
        for x_vel in 0..=x_max {
            if check_for_hit(x_vel, y_vel, x_min, x_max, y_min, y_max) {
                count += 1;
            }
        }
    }
    count
}

pub fn main() {
    let p1_timer = Instant::now();
    let p1_result = part1(-150, -108);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(81, 129, -150, -108);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time); 
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day16_test1() {
        assert_eq!(part1(-10, -5), 45);
	}

    #[test]
    fn day16_test2() {
        assert_eq!(part2(20, 30, -10, -5), 112);
	}
}
