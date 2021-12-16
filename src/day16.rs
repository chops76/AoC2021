use std::io::Read;
use std::fs::File;
use std::time::Instant;

fn parse_input(path: &str) -> Vec<u8> {
    let mut fstr = String::new();

    let mut ret_vec = Vec::new();
    File::open(path).unwrap().read_to_string(&mut fstr).unwrap();

    for i in 0..fstr.len() {
        let mut val = u8::from_str_radix(&fstr[i..i+1], 16).unwrap();
        let mut tmp = Vec::new();
        for j in 0..4 {
            tmp.push(val % 2);
            val /= 2;
        }
        tmp.reverse();
        ret_vec.append(&mut tmp);
    }
    
    ret_vec
}

fn read_n_bits(binary: &Vec<u8>, pos: &mut usize, bits: usize) -> u64 {
    let mut ret_val: u64 = 0;
    for _ in 0..bits {
        ret_val *= 2;
        ret_val += binary[*pos] as u64;
        *pos += 1;
    }
    ret_val as u64
}

fn read_literal(binary: &Vec<u8>, pos: &mut usize) -> u64 {
    let mut finished = false;
    let mut ret_val: u64 = 0;
    while !finished {
        finished = binary[*pos] == 0;
        *pos += 1;
        for _ in 0..4 {
            ret_val *= 2;
            ret_val += binary[*pos] as u64;
            *pos += 1;
        }        
    }
    ret_val
}

fn read_packet_version(binary: &Vec<u8>, pos: &mut usize) -> usize {
    let mut sum = 0;
    let version = read_n_bits(binary, pos, 3);
    sum += version as usize;
    let packet_type = read_n_bits(binary, pos, 3);

    if packet_type == 4 {
        read_literal(binary, pos);
    } else {
        let length_is_in_bits = binary[*pos] == 0;
        *pos += 1;
        if length_is_in_bits {
            let length = read_n_bits(binary, pos, 15);
            let target_pos = *pos + length as usize;
            while *pos < target_pos {
                sum += read_packet_version(binary, pos);
            }
        } else {
            let num_packets = read_n_bits(binary, pos, 11);
            for _ in 0..num_packets {
                sum += read_packet_version(binary, pos);
            }
        }
    }
    sum
}

fn read_packet(binary: &Vec<u8>, pos: &mut usize) -> usize {
    read_n_bits(binary, pos, 3);
    let packet_type = read_n_bits(binary, pos, 3);
    
    if packet_type == 4 {
        return read_literal(binary, pos) as usize;
    }
    let mut vals = Vec::new();
    let length_is_in_bits = binary[*pos] == 0;
    *pos += 1;
    if length_is_in_bits {
        let length = read_n_bits(binary, pos, 15);
        let target_pos = *pos + length as usize;
        while *pos < target_pos {
            vals.push(read_packet(binary, pos));
        }
    } else {
        let num_packets = read_n_bits(binary, pos, 11);
        for _ in 0..num_packets {
            vals.push(read_packet(binary, pos));
        }
    }

    match packet_type {
        0 => vals.iter().sum(),
        1 => vals.iter().product(),
        2 => *vals.iter().min().unwrap(),
        3 => *vals.iter().max().unwrap(),
        5 => if vals[0] > vals[1] { 1 } else { 0 },
        6 => if vals[0] < vals[1] { 1 } else { 0 },
        7 => if vals[0] == vals[1] { 1 } else { 0 }
        _ => 0
    }
}

fn part1(binary: &Vec<u8>) -> usize {
    let mut pos = 0;
    read_packet_version(binary, &mut pos)
}

fn part2(binary: &Vec<u8>) -> usize {
    let mut pos = 0;
    read_packet(binary, &mut pos)
}

pub fn main() {
    let binary = parse_input("./input/day16/input.txt");

    let p1_timer = Instant::now();
    let p1_result = part1(&binary);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(&binary);
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
