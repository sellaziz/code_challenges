use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::process::exit;

fn print_map(input: &Vec<Vec<char>>) {
    print!(" ");
    for i in 0..input[0].len() {
        print!("{}", i);
    }
    println!();
    for (i, line) in input.iter().enumerate() {
        print!("{}", i);
        for c in line.iter() {
            print!("{}", c);
        }
        println!();
    }
}

fn update_position(
    x: usize,
    y: usize,
    dx: i32,
    dy: i32,
    max_x: usize,
    max_y: usize,
) -> Option<(usize, usize)> {
    let (mut is_new_x_valid, mut is_new_y_valid) = (true, true);
    let new_x: i32 = x as i32 + dx;
    let new_y: i32 = y as i32 + dy;
    if new_x < 0 || new_x >= max_x as i32 {
        is_new_x_valid = false;
    }
    if new_y < 0 || new_y >= max_y as i32 {
        is_new_y_valid = false;
    }
    if is_new_x_valid && is_new_y_valid {
        Some((new_x as usize, new_y as usize))
    } else {
        None
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

    let original_list: Vec<String> = reader.lines().map_while(Result::ok).collect();
    let mut area_map: Vec<Vec<char>> = original_list
        .iter()
        .map(|line| line.chars().collect())
        .collect();
    // let original_area_map = area_map.clone();
    let max_x = area_map[0].len();
    let max_y = area_map.len();
    let mut chars_positions = HashMap::new();

    area_map.iter().enumerate().for_each(|(i, line)| {
        line.iter().enumerate().for_each(|(j, &v)| {
            chars_positions.entry(v).or_insert(Vec::new()).push((j, i));
        })
    });

    // print_map(&area_map);
    // println!("HashMap: {:?}", chars_positions);

    if part == "part1" {
        let mut counter = 0;
        let mut antinodes = HashMap::new();

        chars_positions
            .iter()
            .filter(|(k, _v)| **k != '.')
            .for_each(|(_k, v)| {
                for i in 0..v.len() {
                    for j in i + 1..v.len() {
                        let (dx, dy): (i32, i32) =
                            (v[j].0 as i32 - v[i].0 as i32, v[j].1 as i32 - v[i].1 as i32);
                        if let Some(update_position) =
                            update_position(v[i].0, v[i].1, -dx, -dy, max_x, max_y)
                        {
                            counter += 1;
                            *antinodes.entry(update_position).or_insert(0) += 1;
                            area_map[update_position.1][update_position.0] = '#';
                        }
                        if let Some(update_position) =
                            update_position(v[j].0, v[j].1, dx, dy, max_x, max_y)
                        {
                            counter += 1;
                            *antinodes.entry(update_position).or_insert(0) += 1;
                            area_map[update_position.1][update_position.0] = '#';
                        }
                        // println!("---");
                        // print_map(&area_map);
                    }
                }
            });
        // println!("antinodes: {:?}", antinodes);
        println!("Solution part 1: {}", antinodes.len());
    } else if part == "part2" {
        // println!("Solution part 2: {}", counter);
    }
}
