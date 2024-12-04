use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::process::exit;

fn transpose_strings(input: Vec<String>) -> Vec<String> {
    // We suppose all String have the same length
    let mut out_vec: Vec<String> = Vec::new();
    for j in 0..input[0].len() {
        let mut vert_string: String = String::new();
        (0..input.len()).for_each(|i: usize| {
            if let Some(c) = input[i].chars().nth(j) {
                vert_string.push(c); // Append the character to vert_string
            }
        });
        out_vec.push(vert_string.to_owned());
    }
    out_vec
}

fn diagonals_string(input: Vec<String>) -> Vec<String> {
    // We suppose all String have the same length
    let mut out_vec: Vec<String> = Vec::new();
    let rows = input.len();
    let cols = input[0].len();
    // diagonal
    (0..(rows + cols - 1)).for_each(|d| {
        let mut diagonal = String::new();
        (0..rows).for_each(|i| {
            let j = d as isize - i as isize;
            if j >= 0 && (j as usize) < cols {
                if let Some(c) = input[i].chars().nth(j as usize) {
                    diagonal.push(c); // Append the character to vert_string
                }
            }
        });
        if !diagonal.is_empty() {
            out_vec.push(diagonal);
        }
    });
    // anti diagonal
    (0..(rows + cols - 1)).for_each(|d| {
        let mut diagonal = String::new();
        (0..rows).for_each(|i| {
            let j = i as isize + d as isize - (cols as isize - 1);
            if j >= 0 && (j as usize) < cols {
                if let Some(c) = input[i].chars().nth(j as usize) {
                    diagonal.push(c); // Append the character to vert_string
                }
            }
        });
        if !diagonal.is_empty() {
            out_vec.push(diagonal);
        }
    });
    out_vec
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
    // println!("Number List: {:?}", number_list);

    if part == "part1" {
        let mut reversed_list = list.clone();
        reversed_list = reversed_list
            .iter()
            .map(|s| s.chars().rev().collect::<String>())
            .collect();

        let mut vertical_list = list.clone();
        vertical_list = transpose_strings(vertical_list);

        let mut rev_vertical_list = vertical_list.clone();
        rev_vertical_list = rev_vertical_list
            .iter()
            .map(|s| s.chars().rev().collect::<String>())
            .collect();

        let mut diagonals = list.clone();
        diagonals = diagonals_string(diagonals);

        let mut rev_diagonals = diagonals.clone();
        rev_diagonals = rev_diagonals
            .iter()
            .map(|s| s.chars().rev().collect::<String>())
            .collect();

        let all_combinations: Vec<String> = vec![
            list,
            reversed_list,
            vertical_list,
            rev_vertical_list,
            diagonals,
            rev_diagonals,
        ]
        .into_iter()
        .flatten()
        .collect();
        let result: usize = all_combinations
            .iter()
            .map(|s| s.matches("XMAS").count())
            .sum();
        println!("Solution Part 1: {}", result);
    } else if part == "part2" {
    }
}
