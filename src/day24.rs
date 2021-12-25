use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;
use std::collections::VecDeque;

#[derive(Debug)]
#[derive(Clone)]
struct Entry {
	ins: String,
	param1: String,
    param2: String
}

type Input = Vec<Entry>;

fn parse_line(s: &str) -> Entry {
    let spl: Vec<&str> = s.split(" ").collect();
	Entry {
		ins: spl[0].to_string(),
		param1: spl[1].to_string(),
        param2: if spl[0] == "inp" { String::new() } else { spl[2].to_string() }
	}
}

fn parse_input(path: &str) -> Input {
    let f = File::open(path).unwrap();
    BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn reg_num(reg: &str) -> usize {
    match reg {
        "w" => 0,
        "x" => 1,
        "y" => 2, 
        "z" => 3,
        _ => 4
    }
}

fn check_fast(val: u64) -> bool {
    let mut inputs = VecDeque::new();
    let mut digits = val;
    for _ in 0..7 {
        inputs.push_front((digits % 10) as i64);
        digits /= 10;
    }
    if inputs.contains(&0) {
        return false;
    }
    let mut w = 0;
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;

    w = inputs[0];
    if val == 1172151 {
        println!("{}  {}", w, z);
    }
    
    if w != 15 {
        z *= 26;
        z += w + 13;
    }

    //println!("At input {} {} {} {}", w, x, y, z);

    w = inputs[1];
    if val == 1172151 {
        println!(" {}  {}", w, z);
    }
    if w != z % 26 + 10 {
        z *= 26;
        z += w + 16;
    }

    //println!("At input {} {} {} {}", w, x, y, z);
    w = inputs[2];
    if val == 1172151 {
        println!("{}  {}", w, z);
    }
    if w != z % 26 + 12 {
        z *= 26;
        z += w + 2;
    }
    
    //println!("At input {} {} {} {}", w, x, y, z);

    w = inputs[3];
    if val == 1172151 {
        println!("{}  {}", w, z);
    }
    if w != z % 26 + 10 {
        z *= 26;
        z += w + 8;
    }

    //println!("At input {} {} {} {}", w, x, y, z);

    w = inputs[4];
    if val == 1172151 {
        println!("{}  {}", w, z);
    }
    if w != z % 26 + 14 {
        z *= 26;
        z += w + 11;
    }

    w = z % 26 - 11;
    if val == 1172151 {
        println!("{}  {}", w, z);
    }
    if w < 1 || w > 9 {
        return false;
    }
    let tmp = z;
    z /= 26;
    if w != tmp % 26 - 11 {
        z *= 26;
        z += w + 6;
    } else {
        //println!("Check good!");
    }

    w = inputs[5];
    if val == 1172151 {
        println!("{}  {}", w, z);
    }

    if w != z % 26 + 10 {
        z *= 26;
        z += w + 12;
    }

    w = z % 26 - 16;
    if val == 1172151 {
        println!("{}  {}", w, z);
    }
    if w < 1 || w > 9 {
        return false;
    }
    let tmp = z;
    z /= 26;
    if w != tmp % 26 - 16 {
        z *= 26;
        z += w + 2;
    } else {
        //println!("Check Passed");
    }
    //println!("Made it here");

    w = z % 26 - 9;
    if val == 1172151 {
        println!("{}  {}", w, z);
    }
    if w < 1 || w > 9 {
        return false;
    }
    let tmp = z;
    z /= 26;
    if w != tmp % 26 - 9 {
        z *= 26;
        z += w + 2;
    } else {
        //println!("Check passed");
    }

    //println!("First OK");

    w = inputs[6];
    if val == 1172151 {
        println!("{}  {}", w, z);
    }

    if w != z % 26 + 11 {
        z *= 26;
        z += w + 15;
    }
    //println!("At input {} {} {} {}", w, x, y, z);

    w = z % 26 - 8;
    if w < 1 || w > 9 {
        return false;
    }
    if val == 1172151 {
        println!("{}  {}", w, z);
    }
    let tmp = z;
    z /= 26;
    if w != tmp % 26 - 8 {
        z *= 26;
        z += w + 1;
    }

    //println!("At input {} {} {} {}", w, x, y, z);

    w = z % 26 - 8;

    if w < 1 || w > 9 {
        return false;
    }
    if val == 1172151 {
        println!("{}  {}", w, z);
    }
    let tmp = z;
    z /= 26;
    if w != tmp % 26 - 8 {
        z *= 26;
        z += w + 10;
    }

    //println!("At input {} {} {} {}", w, x, y, z);   

    w = z % 26 - 10;

    if w < 1 || w > 9 {
        return false;
    }
    if val == 1172151 {
        println!("{}  {}", w, z);
    }
    let tmp = z;
    z /= 26;
    if w != tmp % 26 - 10 {
        z *= 26;
        z += w + 14;
    }

    //println!("At input {} {} {} {}", w, x, y, z);   

    w = z % 26 - 9;

    if w < 1 || w > 9 {
        return false;
    }
    if val == 1172151 {
        println!("{}  {}", w, z);
    }
    let tmp = z;
    z /= 26;
    if w != tmp % 26 - 9 {
        z *= 26;
        z += w + 10;
    }   
    if z == 0 {
        println!("Done:  {}", val);   
    }

    z == 0
}

fn check(prog: &Input, val: u64) -> bool {
    let mut inputs = VecDeque::new();
    let mut digits = val;
    for _ in 0..14 {
        inputs.push_front((digits % 10) as i64);
        digits /= 10;
    }

    let mut regs: Vec<i64> = vec![0, 0, 0, 0];
    let mut input_ptr = 0;
    for inst in prog {
        let param1_reg_num = reg_num(&inst.param1);
        let param2_reg_num = reg_num(&inst.param2);
        let param1_value = regs[param1_reg_num];
        let param2_value = match param2_reg_num {
            4 => if inst.ins == "inp" { 0 } else { inst.param2.parse().unwrap() },
            _ => regs[param2_reg_num]
        };
        match inst.ins.as_str() {
            "inp" => { println!("Before input {:?}", regs);  regs[param1_reg_num] = inputs[input_ptr]; input_ptr += 1 },
            "add" => { regs[param1_reg_num] = param1_value + param2_value },
            "mul" => { regs[param1_reg_num] = param1_value * param2_value },
            "div" => { regs[param1_reg_num] = param1_value / param2_value },
            "mod" => { regs[param1_reg_num] = param1_value % param2_value },
            "eql" => { regs[param1_reg_num] = if param1_value == param2_value { 1 } else { 0 }},
            _ => println!("Illegal Instruction {}", inst.ins)
        }
    }

    println!("Final: {:?}", regs);
    regs[3] == 0
}

fn part1(prog: &Input) -> u64 {
    let mut cur_val: u64 = 9999999;
    let mut last_reported: u64 = 99999999999999;

    check(prog, 53999995829399);
//    check_fast(cur_val);

    while cur_val >= 1111111 {
        if check_fast(cur_val) {
            return cur_val;
        }
        cur_val -= 1;
    } 
    0
}

fn part2(prog: &Input) -> u64 {
    let mut cur_val: u64 = 1111111;
    let mut last_reported: u64 = 99999999999999;

//    check(prog, 53999995829399);
//    check_fast(cur_val);

    while cur_val <= 9999999 {
        if check_fast(cur_val) {
            return cur_val;
        }
        cur_val += 1;
    } 
    0
}

pub fn main() {
	let input = parse_input("./input/day24/input.txt");

	let p1_timer = Instant::now();
	let p1_result = part1(&input);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&input);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time); 
}