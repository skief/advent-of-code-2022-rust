use std::fs;
use crate::common::Solution;

fn solve(input: &Vec<i32>) -> Vec<i32> {
    let mut calories: Vec<i32> = Vec::new();
    let mut sum = 0;

    for i in 0..input.len() {
        if input[i] != -1 {
            sum += input[i];
        } else {
            calories.push(sum);
            sum = 0;
        }
    }
    calories.push(sum);

    calories
}

fn part1(input: &Vec<i32>) -> i32 {
    let calories = solve(input);

    *calories.iter().max().unwrap()
}

fn part2(input: &Vec<i32>) -> i32 {
    let mut calories = solve(input);
    calories.sort();
    calories.reverse();

    calories[0] + calories[1] + calories[2]
}

pub fn run() -> (Solution, Solution) {
    let values: Vec<i32> = fs::read_to_string("inputs/day01.txt")
        .unwrap()
        .lines()
        .map(|x| x.parse().unwrap_or(-1))
        .collect();

    (Solution::from(part1(&values)),
     Solution::from(part2(&values)))
}
