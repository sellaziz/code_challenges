use std::collections::HashMap;
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
    let mut number_list: Vec<Vec<i32>> = list
        .iter()
        .map(|line| {
            line.split(' ')
                .filter_map(|value| value.parse::<i32>().ok())
                .collect()
        })
        .collect();
    // println!("List: {:?}", list);
    // println!("Number List: {:?}", number_list);

    if part == "part1" {
        let distances: Vec<Vec<i32>> = number_list
            .iter()
            .map(|vector| {
                vector
                    .windows(2)
                    .map(|window| window[0] - window[1])
                    .collect()
            })
            .collect();
        // println!("Distances: {:?}", distances);
        let condition_diff: Vec<bool> = distances
            .iter()
            .map(|v| v.iter().all(|x| (x.abs() <= 3 && x.abs() >= 1)))
            .collect();
        // println!("Amplitude: {:?}", condition_diff);
        let all_decrease: Vec<bool> = distances.iter().map(|v| v.iter().all(|x| *x < 0)).collect();
        let all_increase: Vec<bool> = distances.iter().map(|v| v.iter().all(|x| *x > 0)).collect();
        // println!("all_decrease: {:?}", all_decrease);
        // println!("all_increase: {:?}", all_increase);
        let same_evol: Vec<bool> = all_decrease
            .iter()
            .zip(all_increase.iter())
            .map(|(a, b)| (a | b))
            .collect();
        // println!("same_evol: {:?}", same_evol);
        let all_cond: Vec<bool> = same_evol
            .iter()
            .zip(condition_diff.iter())
            .map(|(a, b)| (a & b))
            .collect();
        // println!("all_cond: {:?}", all_cond);
        let count_true = all_cond.iter().filter(|&&x| x).count();
        println!("Number of true values: {}", count_true);
    } else if part == "part2" {
    }
}
