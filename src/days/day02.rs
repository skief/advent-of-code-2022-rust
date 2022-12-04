use std::fs;
use crate::Solution;

const ROCK: u32 = 0;
const PAPER: u32 = 1;
const SCISSOR: u32 = 2;

#[derive(Debug)]
struct Game {
    opponent: u32,
    own: u32
}

fn eval(strategy: &Game) -> u32 {
    let mut score = strategy.own + 1;
    if strategy.own == strategy.opponent {
        // Draw
        score += 3;
    } else if strategy.own == (strategy.opponent + 1) % 3 {
        // win
        score += 6;
    }

    score
}

fn solve(input: &Vec<Game>) -> u32 {
    input
        .iter()
        .map(|x| eval(x))
        .sum()
}

fn part1(input: &Vec<Game>) -> u32 {
    solve(input)
}

fn part2(input: &Vec<Game>) -> u32 {
    let n = input
        .iter()
        .map(|s| {
            let mut new_strat = Game {own: 0, opponent: s.opponent};

            if s.own == ROCK {
                // Loose
                new_strat.own = (new_strat.opponent + 2) % 3;
            } else if s.own == PAPER {
                // Draw
                new_strat.own = s.opponent;
            } else {
                // Win
                new_strat.own = (&new_strat.opponent + 1) % 3;
            }

            new_strat
        }).collect();

    solve(&n)
}

fn letter_to_move(letter: &String) -> u32 {
    return match letter.as_str() {
        "A" => ROCK,
        "B" => PAPER,
        "C" => SCISSOR,
        "X" => ROCK,
        "Y" => PAPER,
        "Z" => SCISSOR,
        _ => {panic!("Warn {letter}")}
    };
}

pub fn run() -> (Solution, Solution) {
    let input: Vec<Game> = fs::read_to_string("inputs/day02.txt")
        .unwrap()
        .lines()
        .map(|x| {
            let temp: Vec<String> = x.split_whitespace().map(String::from).collect();
            Game {
                opponent: letter_to_move(&temp[0]),
                own: letter_to_move(&temp[1])
            }
        })
        .collect();

    (Solution::from(part1(&input)),
     Solution::from(part2(&input)))
}
