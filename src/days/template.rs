use std::fs;
use crate::Solution;

fn part1(input: &Vec<String>) -> u32 {
    0
}

fn part2(input: &Vec<String>) -> u32 {
    0
}

pub fn run() -> (Solution, Solution) {
    let input: Vec<String> = fs::read_to_string("inputs/dayXX.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    (Solution::from(part1(&input)),
     Solution::from(part2(&input)))
}
