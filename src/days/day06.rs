use std::fs;
use crate::Solution;

fn solve(input: &Vec<u8>, len: &usize) -> usize {
    for i in *len..input.len(){
        let mut set: Vec<&u8> = input[i-len..i].iter().collect();
        set.sort();
        set.dedup();

        if set.len() == *len {
            return i
        }
    }
    0
}

fn part1(input: &Vec<u8>) -> usize {
    solve(input, &4)
}

fn part2(input: &Vec<u8>) -> usize {
    solve(input, &14)
}

pub fn run() -> (Solution, Solution) {
    let input: Vec<u8> = fs::read_to_string("inputs/day06.txt")
        .unwrap()
        .bytes()
        .collect();

    (Solution::from(part1(&input)),
     Solution::from(part2(&input)))
}
