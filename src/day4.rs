use std::io::Read;
use std::fs::File;
use std::time::Instant;

type Input = (Vec<u64>, Vec<Vec<u64>>);

fn parse_input(path: &str) -> Input {
    let mut fstr = String::new();

    File::open(path).unwrap().read_to_string(&mut fstr).unwrap();
    let spl = fstr.split("\n\n").collect::<Vec<&str>>();
    let picks = spl[0].split(",").map(|s| s.parse().unwrap()).collect::<Vec<u64>>();
    
    let mut boards = Vec::new();
    for i in 1..spl.len() {
        boards.push(spl[i].split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<u64>>());
    }
    (picks, boards)
}

fn is_winner(board: &Vec<u64>) -> bool {
    for i in 0..5 {
        if board[i*5] == 100 && board[i*5+1] == 100 && board[i*5+2] == 100 && 
           board[i*5+3] == 100 && board[i*5+4] == 100 {
               return true; // Row
        }
        if board[i] == 100 && board[i+5] == 100 && board[i+10] == 100 &&
           board[i+15] == 100 && board[i+20] == 100 {
               return true;  // Column
           }
    }

    false
}

fn part1(picks: &Vec<u64>, boards: &Vec<Vec<u64>>) -> u64 {
    let mut marked_boards = boards.clone();

    for pick in picks {
        for board in &mut marked_boards {
            match board.iter().position(|v| v == pick) {
                Some(pos) => {
                    board[pos] = 100;
                    if is_winner(&board) {
                        return board.iter().filter(|&v| *v != 100).sum::<u64>() * pick;
                    }
                },
                None => {}
            };
        }
    }

    0
}

fn part2(picks: &Vec<u64>, boards: &Vec<Vec<u64>>) -> u64 {
    let mut marked_boards = boards.clone();
    let mut winners = 0;

    for pick in picks {
        for board in &mut marked_boards {
            if board[0] == 200 {
                continue;
            }
            match board.iter().position(|v| v == pick) {
                Some(pos) => {
                    board[pos] = 100;
                    if is_winner(&board) {
                        winners += 1;
                        if winners == boards.len() {
                            return board.iter().filter(|&v| *v != 100).sum::<u64>() * pick;
                        } 
                        board[0] = 200;
                    }
                },
                None => {}
            };
        }
    }

    0
}

pub fn main() {
    let (picks, boards) = parse_input("./input/day4/input.txt");

    let p1_timer = Instant::now();
    let p1_result = part1(&picks, &boards);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(&picks, &boards);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time); 
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day1_test1() {
        let (picks, boards) = parse_input("./input/day4/test.txt");
        assert_eq!(part1(&picks, &boards), 4512);
	}

    #[test]
    fn day1_test2() {
        let (picks, boards) = parse_input("./input/day4/test.txt");
        assert_eq!(part2(&picks, &boards), 1924);
	}

}
