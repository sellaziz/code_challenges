use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::process::exit;

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

    // Join all lines into one large string
    let mut all_lines = list.join("");
    // println!("Before cleanup: {:?}", all_lines);

    // Regex to clean "dont't()...do()" blocks
    if part == "part2" {
        let re_do = Regex::new(r#"don't\(\).*?do\(\)"#).unwrap();
        all_lines = re_do.replace_all(&all_lines, "").into_owned();
        // println!("After cleanup: {:?}", all_lines);
    }

    // Regex to capture valid "mul(...)" patterns
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    // Extract all captures as tuples
    let all_captures: Vec<(i32, i32)> = re
        .captures_iter(&all_lines)
        .filter_map(|caps| {
            let a = caps[1].parse::<i32>().ok();
            let b = caps[2].parse::<i32>().ok();
            a.zip(b)
        })
        .collect();

    if part == "part1" {
        // Calculate product of all valid pairs
        let mul_result: Vec<i32> = all_captures.iter().map(|(a, b)| a * b).collect();
        let res: i32 = mul_result.iter().sum();
        println!("Part 1 Result = {}", res);
    } else if part == "part2" {
        let mul_result: Vec<i32> = all_captures.iter().map(|(a, b)| a * b).collect();
        let res: i32 = mul_result.iter().sum();
        println!("Part 2 Result = {}", res);
    }
}

