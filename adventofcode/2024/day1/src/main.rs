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

    let (mut list1, mut list2): (Vec<i32>, Vec<i32>) = reader
        .lines()
        .filter_map(|line| {
            line.ok().and_then(|line| {
                // Split the line into two parts
                line.split_once("   ").and_then(|(first, second)| {
                    // Parse both numbers
                    let num1 = first.parse::<i32>().ok();
                    let num2 = second.parse::<i32>().ok();
                    // Return a tuple if parsing is successful
                    num1.zip(num2)
                })
            })
        })
        .unzip();

    list1.sort();
    list2.sort();

    if part == "part1" {
        let distance_list: Vec<i32> = list1
            .iter()
            .zip(list2.iter())
            .map(|(a, b)| (a - b).abs())
            .collect();

        // println!("Distances: {:?}", distance_list);

        let total: i32 = distance_list.iter().sum();
        println!("Sum: {}", total);
    } else if part == "part2" {
        let mut counts2 = HashMap::new();

        for &num in &list2 {
            *counts2.entry(num).or_insert(0) += 1;
        }

        // println!("Count 2: {:?}", counts2);

        let similarities: Vec<i32> = list1
            .iter()
            .filter_map(|value| {
                if counts2.contains_key(value) {
                    Some(value * counts2[value])
                } else {
                    None
                }
            })
            .collect();

        // println!("Similarities: {:?}", similarities);

        let total: i32 = similarities.iter().sum();
        println!("Sum: {}", total);
    }
}
