use std::collections::HashMap;
use std::fs;
use crate::Solution;

fn part1(input: &Vec<usize>) -> usize {
    input.iter().filter(|x| **x <= 100000).sum()
}

fn part2(input: &Vec<usize>) -> usize {
    let total_space = 70000000;
    let required_space = 30000000;
    let used_space = input.iter().max().unwrap();
    let unused_space = total_space - used_space;
    let delete = required_space - unused_space;

    *input.iter().filter(|x| **x >= delete).min().unwrap()
}

fn dir_to_string(path: &Vec<String>) -> String {
    path.iter().fold(String::from("/"), |acc, x| acc + "/" + x)
}

fn dir_size(folders: &HashMap<String, Vec<usize>>, folder: &String) -> usize {
    folders.keys()
        .filter(|x| x.starts_with(folder))
        .map(|x| folders.get(x).unwrap())
        .flat_map(|x| x)
        .sum()
}

pub fn run() -> (Solution, Solution) {
    let input: Vec<String> = fs::read_to_string("inputs/day07.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut folders: HashMap<String, Vec<usize>> = HashMap::new();
    let mut current: Vec<String> = Vec::new();

    folders.insert(dir_to_string(&current), Vec::new());

    let mut i = 0;
    'outer: while i < input.len() {
        let line = &input[i];
        if line.chars().next().unwrap() == '$' {
            let parts: Vec<&str> = line.split_whitespace().collect();

            match parts[1] {
                "cd" => {
                    if parts[2] == "/" {
                        current = Vec::new();
                    } else if parts[2] == ".." {
                        current.pop();
                    } else {
                        current.push(String::from(parts[2]));
                    }
                },
                "ls" => {
                    i += 1;
                    while i < input.len() {
                        let line = &input[i];
                        if line.chars().next().unwrap() == '$' {
                            continue 'outer;
                        }

                        let parts: Vec<&str> = line.split_whitespace().collect();
                        let dir = folders.get_mut(&dir_to_string(&current))
                            .unwrap();

                        if parts[0] == "dir" {
                            current.push(String::from(parts[1]));

                            let path = dir_to_string(&current);
                            if !folders.contains_key(&path) {
                                folders.insert(dir_to_string(&current), Vec::new());
                            }

                            current.pop();
                        } else {
                            dir.push(parts[0].parse().unwrap());
                        }
                        i += 1;
                    }
                },
                _ => panic!()
            }
        }

        i += 1;
    }

    let sizes: Vec<usize> = folders.keys().map(|x| dir_size(&folders, x)).collect();

    (Solution::from(part1(&sizes)),
     Solution::from(part2(&sizes)))
}
