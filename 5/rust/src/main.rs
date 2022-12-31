use regex::Regex;
use std::{collections::VecDeque, io};

fn main() {
    let stack_re = Regex::new(r"\[([A-Z ])\]").unwrap();
    let move_re = Regex::new(r"^move (\d+) from (\d+) to (\d+)").unwrap();

    let mut stacks: Vec<VecDeque<char>> = Vec::default();
    let mut moves: Vec<(usize, usize, usize)> = Vec::default();

    for line in io::stdin().lines() {
        let line = line.unwrap();
        if stack_re.is_match(&line) {
            for (start, chr) in line.chars().into_iter().enumerate() {
                if ('A'..='Z').contains(&chr) {
                    while stacks.len() <= (start - 1) / 4 {
                        stacks.push(VecDeque::<char>::default());
                    }
                    stacks[(start - 1) / 4].push_front(chr);
                }
            }
        }
        if move_re.is_match(&line) {
            let caps = move_re.captures(&line).unwrap();
            moves.push((
                caps[1].parse().unwrap(),
                caps[2].parse().unwrap(),
                caps[3].parse().unwrap(),
            ));
        }
    }

    // println!("{:?} {:?}", stacks, moves);
    let mut stacks_p1 = stacks.clone();
    for (qty, src, dst) in moves.iter() {
        for _ in 0..*qty {
            let hold = stacks_p1[src - 1].pop_back().unwrap();
            stacks_p1[dst - 1].push_back(hold);
        }
    }

    println!(
        "Part 1: message {}",
        stacks_p1
            .iter()
            .map(|q| q.back().unwrap())
            .collect::<String>()
    );

    // println!("{:?} {:?}", stacks, moves);
    let mut stacks_p2 = stacks.clone();
    for (qty, src, dst) in moves.iter() {
        let mut crane = VecDeque::new();
        for _ in 0..*qty {
            crane.push_back(stacks_p2[src - 1].pop_back().unwrap());
        }
        for _ in 0..*qty {
            stacks_p2[dst - 1].push_back(crane.pop_back().unwrap());
        }
    }

    println!(
        "Part 2: message {}",
        stacks_p2
            .iter()
            .map(|q| q.back().unwrap())
            .collect::<String>()
    );
}
