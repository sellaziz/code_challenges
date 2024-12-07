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

    let equations: Vec<(i32, Vec<i32>)> = list
        .iter()
        .map(|line| {
            let (res, nums_str) = line.split_once(':').unwrap();
            let nums = String::from(nums_str)
                .split(" ")
                .filter_map(|value| value.parse::<i32>().ok())
                .collect();
            (res.parse::<i32>().unwrap(), nums)
        })
        .collect();

    println!("Equations: {:?}", equations);

    if part == "part1" {
    } else if part == "part2" {
    }
}
