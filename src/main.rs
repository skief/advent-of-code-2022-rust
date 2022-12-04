use std::env;
use crate::common::Solution;

mod days;
mod common;

struct Benchmark {
    result: (Solution, Solution),
    times: Vec<u128>
}

impl Benchmark {
    fn min(&self) -> u128 {
        *self.times.iter().min().unwrap()
    }

    fn max(&self) -> u128 {
        *self.times.iter().max().unwrap()
    }

    fn mean(&self) -> f64 {
        let sum: u128 = self.times.iter().sum();
        sum as f64 / self.times.len() as f64
    }

    fn std(&self) -> f64 {
        let mean = self.mean();
        let sum: f64 = self.times.iter().map(|x| (*x as f64 - mean).powi(2)).sum();

        (sum / (self.times.len() - 1) as f64).sqrt()
    }
}

fn run_day(day: &u32) -> ((Solution, Solution), u128){
    let solver = get_day(day);

    let start = std::time::Instant::now();
    let result = solver();
    let elapsed = start.elapsed().as_micros();

    ((result.0, result.1), elapsed)
}

fn benchmark_day(day: &u32, num_runs: &usize) -> Benchmark {
    let result = run_day(day).0;
    let times: Vec<u128> = (0..*num_runs).map(|_| run_day(day).1).collect();

    Benchmark { result, times }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide the day(s) to run as a command-line argument.");
    }

    if args[1] != "benchmark" {
        let days: Vec<u32> = args[1..]
            .iter()
            .map(|x| x.parse().unwrap())
            .collect();

        for day in days {
            let (solution, time) = run_day(&day);

            println!("Day {day:02}");
            println!("Part 1: {}", solution.0);
            println!("Part 2: {}", solution.1);
            println!("Time: {:.2}ms", time as f64 / 1000.0);
            println!();
        }

    } else {
        let days: Vec<u32> = args[2..]
            .iter()
            .map(|x| x.parse().unwrap())
            .collect();

        let benchmarks: Vec<Benchmark> = days
            .iter()
            .map(|x| benchmark_day(x, &100))
            .collect();

        let mut total_min = 0;
        let mut total_max = 0;
        let mut total_mean = 0.0;

        for (day, benchmark) in days.iter().zip(benchmarks.iter()) {
            println!("Day {day:02}");
            println!("Part 1: {}", benchmark.result.0);
            println!("Part 2: {}", benchmark.result.1);
            println!("Mean: {:.2}ms Min: {:.2}ms Max: {:.2}ms Std: {:.2}ms",
                     benchmark.mean() as f64 / 1000.0,
                     benchmark.min() as f64 / 1000.0,
                     benchmark.max() as f64 / 1000.0,
                     benchmark.std() as f64 / 1000.0);

            total_min += benchmark.min();
            total_max += benchmark.max();
            total_mean += benchmark.mean();

            println!();
        }

        println!("Total");
        println!("Mean: {:.2}ms Min: {:.2}ms Max: {:.2}ms",
                 total_mean as f64 / 1000.0,
                 total_min as f64 / 1000.0,
                 total_max as f64 / 1000.0);
    }
}

fn get_day(day: &u32) -> fn() -> (Solution, Solution) {
    match day {
        1 => days::day01::run,
        2 => days::day02::run,
        3 => days::day03::run,
        4 => days::day04::run,
        _ => panic!("Unimplemented day")
    }
}
