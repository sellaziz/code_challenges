use itertools::Itertools;
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
    let ordering_rules: Vec<(i32, i32)> = list
        .iter()
        .filter(|&line| line.contains("|"))
        .map(|line| {
            let values: Vec<&str> = line.split('|').collect();
            let a = values.first().unwrap().parse::<i32>().unwrap();
            let b = values.get(1).unwrap().parse::<i32>().unwrap();
            (a, b)
        })
        .collect();
    // println!("Rules: {:?}", ordering_rules);
    let updates: Vec<Vec<i32>> = list
        .iter()
        .filter(|&line| line.contains(","))
        .map(|line| {
            line.split(',')
                .filter_map(|value| value.parse::<i32>().ok())
                .collect()
        })
        .collect();
    // println!("Updates: {:?}", updates);

    if part == "part1" {
        let valid_list: Vec<bool> = updates
            .iter()
            .map(|vector| {
                let mut pairs: Vec<(i32, i32)> = Vec::new();
                for i in 0..vector.len() {
                    for j in i + 1..vector.len() {
                        pairs.push((vector[i], vector[j]));
                    }
                }
                pairs
                    .iter()
                    .map(|(a, b)| {
                        ordering_rules
                            .iter()
                            .filter_map(|(first, second)| {
                                if a == first && b == second {
                                    Some(true)
                                } else if b == first && a == second {
                                    Some(false)
                                } else {
                                    None
                                }
                            })
                            .all(|x| x)
                    })
                    .all(|x| x)
            })
            .collect();
        let valid_vector: Vec<Vec<i32>> = valid_list
            .iter()
            .zip(updates.iter())
            .filter_map(|(cond, vec)| if *cond { Some(vec.clone()) } else { None })
            .collect();
        // println!("Valid: {:?}", valid_list);
        // println!("Valid vectors: {:?}", valid_vector);
        let result: i32 = valid_vector.iter().map(|vec| vec[vec.len() / 2]).sum();
        println!("Solution Part 1: {}", result);
    } else if part == "part2" {
        let valid_list: Vec<bool> = updates
            .iter()
            .map(|vector| {
                let mut pairs: Vec<(i32, i32)> = Vec::new();
                for i in 0..vector.len() {
                    for j in i + 1..vector.len() {
                        pairs.push((vector[i], vector[j]));
                    }
                }
                pairs
                    .iter()
                    .map(|(a, b)| {
                        ordering_rules
                            .iter()
                            .filter_map(|(first, second)| {
                                if a == first && b == second {
                                    Some(true)
                                } else if b == first && a == second {
                                    Some(false)
                                } else {
                                    None
                                }
                            })
                            .all(|x| x)
                    })
                    .all(|x| x)
            })
            .collect();
        let unvalid_vector: Vec<Vec<i32>> = valid_list
            .iter()
            .zip(updates.iter())
            .filter_map(|(cond, vec)| if !*cond { Some(vec.clone()) } else { None })
            .collect();
        // Generate all permutations and find the one that is valid
        let reordered_vector: Vec<Vec<i32>> = unvalid_vector
            .iter()
            .map(|vector| {
                vector
                    .iter()
                    .permutations(vector.len())
                    .map(|perm| perm.into_iter().cloned().collect())
                    .collect()
            })
            .map(|vectors: Vec<Vec<i32>>| {
                vectors
                    .iter()
                    .filter(|&vector| {
                        let mut pairs: Vec<(i32, i32)> = Vec::new();
                        for i in 0..vector.len() {
                            for j in i + 1..vector.len() {
                                pairs.push((vector[i], vector[j]));
                            }
                        }
                        pairs
                            .iter()
                            .map(|(a, b)| {
                                ordering_rules
                                    .iter()
                                    .filter_map(|(first, second)| {
                                        if a == first && b == second {
                                            Some(true)
                                        } else if b == first && a == second {
                                            Some(false)
                                        } else {
                                            None
                                        }
                                    })
                                    .all(|x| x)
                            })
                            .all(|x| x)
                    })
                    .cloned()
                    .collect::<Vec<Vec<i32>>>()
            })
            .map(|vector| vector[0].clone())
            .collect();
        // println!("Reordered vector: {:?}", reordered_vector);
        let result: i32 = reordered_vector.iter().map(|vec| vec[vec.len() / 2]).sum();
        println!("Solution Part 1: {}", result);
    }
}
