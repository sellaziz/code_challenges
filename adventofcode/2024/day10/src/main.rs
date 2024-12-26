use std::collections::HashSet;
use std::env;
use std::fmt::Display;
use std::fs;
use std::io;
use std::io::BufRead;
use std::process::exit;

fn print_map<T: Display>(input: &Vec<Vec<T>>) {
    input.iter().for_each(|vector| {
        vector.iter().for_each(|v| print!("{}", v));
        println!();
    })
}

fn score(grid: &Vec<Vec<u32>>, r: usize, c: usize) -> i32 {
    let (rows, columns) = (grid.len(), grid[0].len());
    let mut queue = vec![(r, c)];
    let mut seen: HashSet<(usize, usize)> = HashSet::from([(r, c)]);
    let mut summits = 0;

    while !queue.is_empty() {
        let dq = queue.pop();
        if dq.is_some() {
            let (cr, cc) = (dq.unwrap().0, dq.unwrap().1);
            for &(nr, nc) in [
                (cr as i32 - 1, cc as i32),
                (cr as i32, cc as i32 - 1),
                (cr as i32 + 1, cc as i32),
                (cr as i32, cc as i32 + 1),
            ]
            .iter()
            {
                if nr < 0 || nc < 0 || nr >= rows as i32 || nc >= columns as i32 {
                    continue;
                }
                if grid[nr as usize][nc as usize] != grid[cr][cc] + 1 {
                    continue;
                }
                if seen.contains(&(nr as usize, nc as usize)) {
                    continue;
                } else {
                    seen.insert((nr as usize, nc as usize));
                }
                if grid[nr as usize][nc as usize] == 9 {
                    summits += 1;
                } else {
                    queue.push((nr as usize, nc as usize));
                }
            }
        } else {
            continue;
        }
    }
    summits
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
    let number_list: Vec<Vec<u32>> = list
        .iter()
        .map(|line| line.chars().filter_map(|v| v.to_digit(10)).collect())
        .collect();

    println!("Number List: {:?}", number_list);
    print_map(&number_list);

    if part == "part1" {
        let trailheads: Vec<(usize, usize)> = number_list
            .iter()
            .enumerate()
            .flat_map(|(r, line)| {
                line.iter()
                    .enumerate()
                    .filter_map(move |(c, value)| if *value == 0 { Some((r, c)) } else { None })
            })
            .collect();
        println!("trailheads : {:?}", trailheads);
        let total: i32 = trailheads
            .iter()
            .map(|(r, c)| score(&number_list, *r, *c))
            .sum();
        println!("Solution part1: {}", total);
    } else if part == "part2" {
    }
}
