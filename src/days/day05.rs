use std::fs;
use crate::Solution;

#[derive(Debug, Clone, Copy)]
struct Move {
    amount: usize,
    from: usize,
    to: usize
}

fn solve(input: &Vec<Move>, piles: &Vec<Vec<u8>>, rev: bool) -> String {
    let mut piles = piles.clone();

    for m in input {
        let len = piles[m.from - 1].len();
        let moved: Vec<u8> = piles[m.from - 1].drain(len - m.amount..).collect();
        if rev {
            piles[m.to - 1].extend(moved.iter().rev());
        } else {
            piles[m.to - 1].extend(moved);
        }

    }

    let top: Vec<u8> = piles.iter().map(|x| *x.last().unwrap()).collect();
    String::from_utf8(top).unwrap()
}

fn part1(input: &Vec<Move>, piles: &Vec<Vec<u8>>) -> String {
    solve(input, piles, true)
}

fn part2(input: &Vec<Move>, piles: &Vec<Vec<u8>>) -> String {
    solve(input, piles, false)
}

pub fn run() -> (Solution, Solution) {
    let input: Vec<String> = fs::read_to_string("inputs/day05.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let sep = input.iter().position(|x| **x == String::from("")).unwrap();

    let setup = &input[..sep];

    let mut piles: Vec<Vec<u8>> = Vec::new();
    for _ in setup[sep-1].split_whitespace() {
        piles.push(Vec::new());
    }

    for i in (0..sep - 1).rev() {
        let line = setup[i].as_bytes();
        for j in 0..piles.len() {

            if 1 + 4 * j < line.len() && line[1 + 4 * j] != b' ' {
                piles[j].push(line[1 + 4 * j])
            }
        }
    }

    let moves: Vec<Move> = input[sep + 1..]
        .iter()
        .map(|x| {
            let x: Vec<String> = x.split(" ").map(String::from).collect();
            Move {
                amount: x[1].parse().unwrap(),
                from: x[3].parse().unwrap(),
                to: x[5].parse().unwrap()
            }
        }).collect();

    (Solution::from(part1(&moves, &piles)),
     Solution::from(part2(&moves, &piles)))
}