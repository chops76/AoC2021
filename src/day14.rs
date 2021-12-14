use std::io::Read;
use std::fs::File;
use std::time::Instant;
use std::collections::HashMap;

type Input = (String, HashMap<String, String>);

fn parse_input(path: &str) -> Input {
    let mut fstr = String::new();

    File::open(path).unwrap().read_to_string(&mut fstr).unwrap();
    let spl = fstr.split("\n\n").collect::<Vec<&str>>();

    let mut rules = HashMap::new();
    for rule_str in spl[1].split("\n") {
        let rule_spl = rule_str.split(" -> ").collect::<Vec<&str>>();
        rules.insert(rule_spl[0].to_string(), rule_spl[1].to_string());
    }
    
    (spl[0].to_string(), rules)
}

fn apply_rules(rules: &HashMap<String, String>, template: &str) -> String {
    let mut ret_str = String::new();
    for i in 0..template.len() - 1 {
        ret_str += &template[i..i+1];
        if rules.contains_key(&template[i..i+2]) {
            ret_str += &rules[&template[i..i+2]];
        }
    }
    ret_str += &template[template.len() - 1..];
    
    ret_str
}

fn part1(rules: &HashMap<String, String>, template: &str) -> usize {
    let mut work_str = template.to_string();
    for _ in 0..10 {
        work_str = apply_rules(rules, &work_str);
    }

    let mut counts:HashMap<char, usize> = HashMap::new();
    for c in work_str.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    *counts.iter().map(|(_, c)| c).max().unwrap() - *counts.iter().map(|(_, c)| c).min().unwrap()
}

fn part2(rules: &HashMap<String, String>, template: &str) -> usize {

    let mut counts = HashMap::new();
    for i in 0..template.len() - 1 {
        *counts.entry(template[i..i+2].to_string()).or_insert(0) += 1;
    }
    
    for _ in 0..40 {
        let mut new_counts:HashMap<String, usize> = HashMap::new();
        for (item_key, item_count) in &counts {
            if rules.contains_key(item_key) {
                let first_str = item_key[0..1].to_string() + &rules[item_key];
                let second_str = rules[item_key].to_string() + &item_key[1..];
                *new_counts.entry(first_str).or_insert(0) += item_count;
                *new_counts.entry(second_str).or_insert(0) += item_count;
            } else {
                *new_counts.entry(item_key.to_string()).or_insert(0) += item_count;
            }
        }
        counts = new_counts;
    }

    let mut char_counts = HashMap::new();
    for (item_key, item_count) in &counts {
        let chars = item_key.chars().collect::<Vec<char>>();
        let c = chars[1];
        *char_counts.entry(c).or_insert(0) += item_count;
    }
    let first_char = template.chars().next().unwrap();
    *char_counts.get_mut(&first_char).unwrap() += 1;

    *char_counts.iter().map(|(_, c)| c).max().unwrap() - *char_counts.iter().map(|(_, c)| c).min().unwrap()
}

pub fn main() {
    let (template, rules) = parse_input("./input/day14/input.txt");

    let p1_timer = Instant::now();
    let p1_result = part1(&rules, &template);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(&rules, &template);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day14_test1() {
        let (template, rules) = parse_input("./input/day14/test.txt");
        assert_eq!(part1(&rules, &template), 1588);
	}

    #[test]
    fn day14_test2() {
        let (template, rules) = parse_input("./input/day14/test.txt");
        assert_eq!(part2(&rules, &template), 2188189693529);
	}
}
