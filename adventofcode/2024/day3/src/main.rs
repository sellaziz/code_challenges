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

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut all_all_captures: Vec<Vec<(i32, i32)>> = Vec::new();

    (0..list.len()).for_each(|i| {
        let all_line_captures: Vec<(i32, i32)> = re
            .captures_iter(list[i].as_str()) // Get an iterator over all matches
            .filter_map(|caps| {
                let a = caps[1].parse::<i32>().ok(); // Parse the first capture group
                let b = caps[2].parse::<i32>().ok(); // Parse the second capture group
                a.zip(b) // Combine into a tuple if both are valid
            })
            .collect();
        all_all_captures.push(all_line_captures);
    });
    let all_captures: Vec<(i32, i32)> = all_all_captures.into_iter().flatten().collect();
    // println!("out {:?}", all_captures);

    if part == "part1" {
        let mul_result: Vec<i32> = all_captures.iter().map(|(a, b)| a * b).collect();
        let res: i32 = mul_result.iter().sum();
        println!("Result = {}", res);
    } else if part == "part2" {
    }
}
