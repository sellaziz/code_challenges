use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

fn has_hole(input: &Vec<i32>) -> Option<(usize, usize)> {
    let mut last_digit_index: usize = 0;
    let mut previous_hole_index: usize = 0;
    let mut last_hole_index: usize = 0;
    let mut first_hole_index = 0;
    for (i, v) in input.iter().enumerate() {
        if *v == -1 {
            first_hole_index = i;
            break;
        }
    }
    input.iter().enumerate().for_each(|(i, v)| {
        if *v != -1 {
            last_digit_index = i;
            last_hole_index = previous_hole_index;
        } else if *v == -1 {
            previous_hole_index = i;
        }
    });
    if last_digit_index < last_hole_index || last_hole_index == 0 {
        None
    } else {
        Some((first_hole_index, last_digit_index))
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
    let number_list: Vec<u32> = list
        .first()
        .unwrap()
        .chars()
        .filter_map(|v| v.to_digit(10))
        .collect();

    // println!("Number List: {:?}", number_list);

    if part == "part1" {
        let mut representation: Vec<i32> = Vec::new();
        number_list
            .windows(2)
            .step_by(2)
            .enumerate()
            .for_each(|(i, w)| {
                let (blk_file, blk_free) = (w[0], w[1]);
                println!("{}, {}", blk_file, blk_free);
                let tmp_str0 = vec![i as i32].repeat(blk_file as usize);
                let tmp_str1 = vec![-1 as i32].repeat(blk_free as usize);
                tmp_str0.iter().for_each(|c| representation.push(*c));
                tmp_str1.iter().for_each(|c| representation.push(*c));
            });
        let blk_file: u32 = *number_list.last().unwrap();
        let tmp_str0 = vec![(number_list.len() / 2) as i32].repeat(blk_file as usize);
        tmp_str0.iter().for_each(|c| representation.push(*c));
        println!("{:?}", representation);
        while let Some((first_hole_index, last_digit_index)) = has_hole(&representation) {
            representation.swap(first_hole_index, last_digit_index);
            // println!("{}, {}", first_hole_index, last_digit_index);
            // println!("{:?}", representation);
            // sleep(Duration::new(1, 0));
        }
        // println!("{:?}", representation);
        let result: u64 = representation
            .iter()
            .filter(|&c| *c != -1)
            .enumerate()
            .map(|(i, c)| i as u64 * *c as u64)
            .sum();
        println!("Solution Part 1: {}", result);
    } else if part == "part2" {
    }
}
