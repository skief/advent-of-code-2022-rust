use std::fs;
use crate::Solution;

fn part1(input: &Vec<(u32, u32, u32, u32)>) -> u32 {
    input
        .iter()
        .filter(|(a, b, c, d)| (a >= c && b <= d) || (c >= a && d <= b))
        .count() as u32
}

fn part2(input: &Vec<(u32, u32, u32, u32)>) -> u32 {
    input
        .iter()
        .filter(|(a, b, c, d)| a <= d && b >= c)
        .count() as u32
}

pub fn run() -> (Solution, Solution) {
    let input: Vec<(u32, u32, u32, u32)> = fs::read_to_string("inputs/day04.txt")
        .unwrap()
        .lines()
        .map(|x| {
            let elves: Vec<String> = x.split(",").map(String::from).collect();

            let e1: Vec<String> = elves[0].split("-").map(String::from).collect();
            let e2: Vec<String> = elves[1].split("-").map(String::from).collect();
            (e1[0].parse().unwrap(), e1[1].parse().unwrap(),
             e2[0].parse().unwrap(), e2[1].parse().unwrap())
        })
        .collect();

    (Solution::from(part1(&input)),
     Solution::from(part2(&input)))
}
