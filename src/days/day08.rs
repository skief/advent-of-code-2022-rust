use std::fs;
use crate::Solution;

fn is_visible(input: &Vec<Vec<u8>>, y: i32, x:i32) -> bool {
    let height = input[y as usize][x as usize];
    let dirs: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    for dir in dirs {
        let mut pos = (y, x);
        loop {
            pos.0 += dir.0;
            pos.1 += dir.1;
            if pos.0 < 0 || pos.0 >= input.len() as i32 {
                return true;
            }
            if pos.1 < 0 || pos.1 >= input[0].len() as i32 {
                return true;
            }

            if input[pos.0 as usize][pos.1 as usize] >= height {
                break;
            }
        }

    }

    false
}

fn view_distance(input: &Vec<Vec<u8>>, y: i32, x:i32) -> i32 {
    let height = input[y as usize][x as usize];
    let dirs: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut distances: Vec<i32> = Vec::new();

    for dir in dirs {
        let mut dist = 1;
        loop {
            let new_pos = (y + dist * dir.0,
                           x + dist * dir.1);

            if (new_pos.0 < 0 || new_pos.0 >= input.len() as i32) ||
                (new_pos.1 < 0 || new_pos.1 >= input[0].len() as i32) {
                dist -= 1;
                break;
            }

            if input[new_pos.0 as usize][new_pos.1 as usize] >= height {
                break;
            }
            dist += 1;

        }
        distances.push(dist);
    }

    distances.into_iter().reduce(|x, y| x * y).unwrap()
}

fn part1(input: &Vec<Vec<u8>>) -> usize {
    (0..input.len())
        .map(|y| (0..input[1].len())
            .filter(|x| is_visible(input, y as i32, *x as i32))
            .count())
        .sum()
}

fn part2(input: &Vec<Vec<u8>>) -> i32 {
    (0..input.len())
        .map(|y| (0..input[1].len()).map(|x|
            view_distance(input, y.try_into().unwrap(), x.try_into().unwrap()))
            .max().unwrap())
        .max().unwrap()
}

pub fn run() -> (Solution, Solution) {
    let input: Vec<Vec<u8>> = fs::read_to_string("inputs/day08.txt")
        .unwrap()
        .lines()
        .map(|x| x.chars().map(|x| x.to_digit(10).unwrap() as u8).collect())
        .collect();

    (Solution::from(part1(&input)),
     Solution::from(part2(&input)))
}
