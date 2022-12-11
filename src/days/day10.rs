use std::fs;
use crate::Solution;


struct Instruction {
    parameter: i32,
    cycles: i32
}

fn part1(input: &Vec<Instruction>) -> i32 {
    let mut cycle: i32 = 0;
    let mut res: i32 = 0;
    let mut x: i32 = 1;

    for instr in input {
        for _ in 0..instr.cycles {
            cycle += 1;
            if (cycle - 20 ) % 40 == 0 {
                res += x * cycle;
            }
        }
        x += instr.parameter;
    }

    res
}

fn part2(input: &Vec<Instruction>) -> String {
    let mut cycle: i32 = -1;
    let mut res: String = String::new();
    let mut x: i32 = 1;

    for instr in input {
        for _ in 0..instr.cycles {
            cycle += 1;

            let pos = cycle % 40;

            if pos == 0 {
                res += "\n";
            }

            if (x - pos).abs() <= 1 {
                res += "#";
            } else {
                res += ".";
            }
        }
        x += instr.parameter;
    }

    res
}


pub fn run() -> (Solution, Solution) {
    let input: Vec<Instruction> = fs::read_to_string("inputs/day10.txt")
        .unwrap()
        .lines()
        .map( |x| {
            let parts: Vec<String> = x.split_whitespace().map(String::from).collect();
            let cycles = match parts[0].as_str() {
                "addx" => 2,
                "noop" => 1,
                _ => panic!()
            };
            let mut parameter = 0;
            if parts.len() > 1 {
                parameter = parts[1].parse().unwrap();
            }
            Instruction{ parameter, cycles }
        })
        .collect();

    (Solution::from(part1(&input)),
     Solution::from(part2(&input)))
}
