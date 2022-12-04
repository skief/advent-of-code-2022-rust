use std::collections::HashSet;
use std::fs;
use crate::Solution;

fn get_score(c: &u8) -> u32 {
    return if c <= &b'Z' {
        (*c - b'A') as u32 + 27
    } else {
        (*c - b'a') as u32 + 1
    }
}

fn part1(input: &Vec<String>) -> u32 {
    input
        .iter()
        .map(|line| {
            let first: HashSet<&u8> = HashSet::from_iter(
                line[0..line.len() / 2].as_bytes());
            let second: HashSet<&u8> = HashSet::from_iter(
                line[line.len() / 2..line.len()].as_bytes());

            get_score(first.intersection(&second).next().unwrap())
        })
        .sum()
}

fn part2(input: &Vec<String>) -> u32 {
    let elves: Vec<HashSet<&u8>> = input
        .iter()
        .map(|x| HashSet::from_iter(x.as_bytes()))
        .collect();
    let mut score = 0;
    let mut done: Vec<bool> = vec![false; elves.len()];

    'outer: for (i, elve1) in elves.iter().enumerate() {
        if done[i] {
            continue;
        }

        for (j, elve2) in elves.iter().enumerate() {
            if elve1 == elve2 || done[j]{
                continue;
            }

            if !elve1.is_disjoint(elve2) {
                let intersect: HashSet<&u8> = elve1.intersection(elve2).cloned().collect();

                for (k, elve3) in elves.iter().enumerate() {
                    if elve1 == elve3 || elve2 == elve3 || done[k] {
                        continue;
                    }

                    if !intersect.is_disjoint(elve3) {
                        score += get_score(intersect.intersection(elve3).next().unwrap());
                        done[i] = true;
                        done[j] = true;
                        done[k] = true;
                        continue 'outer;
                    }
                }
            }
        }
    }

    score
}

pub fn run() -> (Solution, Solution) {
    let input: Vec<String> = fs::read_to_string("inputs/day03.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    (Solution::from(part1(&input)),
     Solution::from(part2(&input)))
}
