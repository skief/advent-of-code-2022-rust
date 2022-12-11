use std::collections::HashSet;
use std::fs;
use crate::Solution;

const R: i32 = 0;
const L: i32 = 1;
const U: i32 = 2;
const D: i32 = 3;

const DIR: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn solve(input: &Vec<(i32, i32)>, ropes: &usize) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let mut rope: Vec<(i32, i32)> = Vec::with_capacity(*ropes);
    for _ in 0..*ropes {
        rope.push((0, 0));
    }

    for i in input {
        for _ in 0..i.1 {
            rope[0].0 += DIR[i.0 as usize].0;
            rope[0].1 += DIR[i.0 as usize].1;

            for j in 1..*ropes {
                if (rope[j-1].0 - rope[j].0).abs() > 1 || (rope[j-1].1 - rope[j].1).abs() > 1 {
                    rope[j].0 += (rope[j-1].0 - rope[j].0).signum();
                    rope[j].1 += (rope[j-1].1 - rope[j].1).signum();
                }
            }

            visited.insert(rope[rope.len() - 1]);
        }
    }

    visited.len()
}

fn part1(input: &Vec<(i32, i32)>) -> usize {
    solve(input, &2)
}

fn part2(input: &Vec<(i32, i32)>) -> usize {
    solve(input, &10)
}

pub fn run() -> (Solution, Solution) {
    let input: Vec<(i32, i32)> = fs::read_to_string("inputs/day09.txt")
        .unwrap()
        .lines()
        .map(|x| {
            let parts: Vec<String> = x.split_whitespace().map(String::from).collect();
            let dir = match parts[0].as_str() { "R" => R, "L" => L, "U" => U, "D" => D,
                _ => panic!() };
            (dir, parts[1].parse().unwrap())
        })
        .collect();

    (Solution::from(part1(&input)),
     Solution::from(part2(&input)))
}
