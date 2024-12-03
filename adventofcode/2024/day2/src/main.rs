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
    let number_list: Vec<Vec<i32>> = list
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
        let safe_list: Vec<bool> = number_list
            .iter()
            .map(|vector| {
                let mut sorted = vector.clone();
                sorted.sort();
                let mut reverse_sorted = sorted.clone();
                reverse_sorted.reverse();
                vector
                    .windows(2)
                    .map(|window| window[0] - window[1])
                    .all(|x| (1..=3).contains(&x.abs()))
                    && (sorted == *vector || reverse_sorted == *vector)
            })
            .collect();
        let count_true = safe_list.iter().filter(|&&x| x).count();
        println!("Number of true values: {}", count_true);
    } else if part == "part2" {
    }
}
