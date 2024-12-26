use std::collections::HashMap;
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

fn neighbors(
    numbers: &Vec<Vec<u32>>,
    x: usize,
    y: usize,
    max_x: usize,
    max_y: usize,
) -> Vec<(usize, usize)> {
    let potential_neighbors: Vec<(i32, i32)> = vec![
        (x as i32 - 1, y as i32),
        (x as i32, y as i32 - 1),
        (x as i32 + 1, y as i32),
        (x as i32, y as i32 + 1),
    ];
    potential_neighbors
        .iter()
        .filter(|&&(i, j)| i < max_x as i32 && i >= 0 && j < max_y as i32 && j >= 0)
        .filter(|&&(i, j)| numbers[j as usize][i as usize] == numbers[y][x] + 1)
        .map(|&(i, j)| (i as usize, j as usize))
        .collect()
}

fn find_trail(
    numbers: &Vec<Vec<u32>>,
    x: usize,
    y: usize,
    max_x: usize,
    max_y: usize,
    current_trail: &mut Vec<(usize, usize)>,
    trails: &mut Vec<Vec<(usize, usize)>>,
) {
    let potential_neighbors = neighbors(numbers, x, y, max_x, max_y);
    if potential_neighbors.is_empty() && current_trail.len() == 10 {
        trails.push(current_trail.clone());
    } else if potential_neighbors.is_empty() {
        return;
    } else {
        potential_neighbors.iter().for_each(|&(i, j)| {
            let mut new_trail = current_trail.clone();
            new_trail.push((i, j));
            find_trail(numbers, i, j, max_x, max_y, &mut new_trail, trails)
        })
    }
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

    // println!("Number List: {:?}", number_list);
    // print_map(&number_list);

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
        let mut trails: Vec<Vec<(usize, usize)>> = Vec::new();
        number_list.iter().enumerate().for_each(|(j, line)| {
            line.iter().enumerate().for_each(|(i, v)| {
                if *v == 0 {
                    let mut new_trail = vec![(i, j)];
                    find_trail(
                        &number_list,
                        i,
                        j,
                        line.len(),
                        number_list.len(),
                        &mut new_trail,
                        &mut trails,
                    );
                }
            })
        });
        // println!("{:?}", trails);
        // println!("{:?}", trails.len());
        // trails.iter().for_each(|v| {
        //     println!("---");
        //     let mut vec_of_vec: Vec<Vec<char>> =
        //         vec![vec!['.'; number_list.len()]; number_list[0].len()];
        //
        //     v.iter().for_each(|&(i, j)| {
        //         vec_of_vec[j][i] = char::from_digit(number_list[j][i], 10).unwrap()
        //     });
        //     print_map(&vec_of_vec);
        // });
        let mut total = 0;
        let mut trailheads: HashMap<(usize, usize), i32> = HashMap::new();
        number_list.iter().enumerate().for_each(|(j, line)| {
            line.iter().enumerate().for_each(|(i, v)| {
                if *v == 0 {
                    let init_trail = (i, j);
                    let mut count = 0;
                    trails.iter().for_each(|trail| {
                        if init_trail == trail[0] {
                            count += 1;
                        }
                    });
                    if count != 0 {
                        trailheads.insert(init_trail, count);
                    }
                    total += count;
                }
            })
        });
        // println!("trailheads: {:?}", trailheads);
        println!("Solution part 2: {}", total);
    }
}
