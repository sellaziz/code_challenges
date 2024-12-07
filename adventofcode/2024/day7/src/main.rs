use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::process::exit;

fn generate_combinations(
    numbers: &Vec<u64>,
    idx: usize,
    current_result: u64,
    target: u64,
    allow_concat: bool,
    found: &mut bool,
) {
    if idx == numbers.len() {
        // If we have processed all numbers, check if the result matches the target
        if current_result == target {
            *found = true;
        }
        return;
    }

    // Try addition
    generate_combinations(
        &numbers,
        idx + 1,
        current_result + numbers[idx],
        target,
        allow_concat,
        found,
    );

    // Try multiplication
    generate_combinations(
        &numbers,
        idx + 1,
        current_result * numbers[idx],
        target,
        allow_concat,
        found,
    );

    if allow_concat {
        // Try concatenation
        generate_combinations(
            &numbers,
            idx + 1,
            format!("{}{}", current_result, numbers[idx])
                .parse::<u64>()
                .unwrap(),
            target,
            allow_concat,
            found,
        );
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let valid_args: HashSet<&str> = ["part1", "part2"].iter().cloned().collect();

    if args.len() < 3 {
        exit(1);
    }

    let part = &args[1];
    let file_path = &args[2];

    if !valid_args.contains(part.as_str()) {
        exit(1);
    }
    println!("Parsing {file_path}...");

    let contents = fs::File::open(file_path).expect("Should have been able to read the file");
    let reader = io::BufReader::new(contents);

    let list: Vec<String> = reader.lines().map_while(Result::ok).collect();

    let equations: Vec<(u64, Vec<u64>)> = list
        .iter()
        .map(|line| {
            let (res, nums_str) = line.split_once(':').unwrap();
            let nums = String::from(nums_str)
                .split(" ")
                .filter_map(|value| value.parse::<u64>().ok())
                .collect();
            (res.parse::<u64>().unwrap(), nums)
        })
        .collect();

    // println!("Equations: {:?}", equations);

    if part == "part1" {
        let mut valid_eq: Vec<u64> = Vec::new();
        for (res, nums) in equations.iter() {
            let mut found = false;
            generate_combinations(nums, 1, nums[0], *res, false, &mut found);
            if found {
                valid_eq.push(*res);
            }
        }
        println!("Solution Part1: {}", valid_eq.iter().sum::<u64>());
    } else if part == "part2" {
        let mut valid_eq: Vec<u64> = Vec::new();
        for (res, nums) in equations.iter() {
            let mut found = false;
            generate_combinations(nums, 1, nums[0], *res, true, &mut found);
            if found {
                valid_eq.push(*res);
            }
        }
        println!("Solution Part2: {}", valid_eq.iter().sum::<u64>());
    }
}
