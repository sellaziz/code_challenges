use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::process::exit;

#[derive(Debug)]
enum Orientation {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug)]
struct Guard {
    x: usize,
    y: usize,
    orientation: Orientation,
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

fn print_map(input: &Vec<Vec<char>>) {
    for line in input.iter() {
        for c in line.iter() {
            print!("{}", c);
        }
        println!();
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
    let original_area_map = area_map.clone();
    let max_x = area_map[0].len();
    let max_y = area_map.len();

    if part == "part1" {
        let mut counter = 0;
        let mut guard = Guard {
            x: 0,
            y: 0,
            orientation: Orientation::Up,
        };
        let current_position: (i32, i32) = *area_map
            .iter()
            .enumerate()
            .filter_map(|(idx, line)| {
                line.iter()
                    .position(|&x| x == '^')
                    .map(|position| (idx as i32, position as i32))
            })
            .collect::<Vec<(i32, i32)>>()
            .first()
            .unwrap();
        guard.x = current_position.1 as usize;
        guard.y = current_position.0 as usize;
        guard.orientation = match area_map[guard.y].get(guard.x).unwrap() {
            '^' => Orientation::Up,
            'v' => Orientation::Down,
            '>' => Orientation::Right,
            '<' => Orientation::Left,
            _ => exit(1),
        };
        // println!("guard: {:?}", guard);
        loop {
            // println!("round!");
            // println!("guard: {:?}", guard);

            let (dx, dy): (i32, i32) = match guard.orientation {
                Orientation::Up => (0, -1),
                Orientation::Down => (0, 1),
                Orientation::Right => (1, 0),
                Orientation::Left => (-1, 0),
            };
            let update = update_position(guard.x, guard.y, dx, dy, max_x, max_y);
            if update.is_none() {
                area_map[guard.y][guard.x] = 'X';
                counter += 1;
                // println!("--");
                // print_map(&area_map);
                break;
            } else {
                let (nx, ny) = update.unwrap();
                if *area_map[ny].get(nx).unwrap() == '#' {
                    guard.orientation = match guard.orientation {
                        Orientation::Up => Orientation::Right,
                        Orientation::Down => Orientation::Left,
                        Orientation::Right => Orientation::Down,
                        Orientation::Left => Orientation::Up,
                    };
                } else {
                    area_map[guard.y][guard.x] = 'X';
                    guard.x = nx;
                    guard.y = ny;
                    if *area_map[guard.y].get(guard.x).unwrap() != 'X' {
                        counter += 1;
                    }
                }
                match guard.orientation {
                    Orientation::Up => area_map[guard.y][guard.x] = '^',
                    Orientation::Down => area_map[guard.y][guard.x] = 'v',
                    Orientation::Right => area_map[guard.y][guard.x] = '>',
                    Orientation::Left => area_map[guard.y][guard.x] = '<',
                };
            }
            // println!("--");
            // print_map(&area_map);
        }
        println!("Solution part 1: {}", counter);
    } else if part == "part2" {
        let mut counter = 0;
        for i in 0..area_map.len() {
            for j in 0..area_map.len() {
                let mut area_map = original_area_map.clone();
                let mut pass_count: [[i32; 200]; 200] = [[0; 200]; 200];
                if area_map[i][j] == '^' {
                    continue;
                }
                area_map[i][j] = 'O';
                let mut guard = Guard {
                    x: 0,
                    y: 0,
                    orientation: Orientation::Up,
                };
                let current_position: (i32, i32) = *area_map
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, line)| {
                        line.iter()
                            .position(|&x| x == '^')
                            .map(|position| (idx as i32, position as i32))
                    })
                    .collect::<Vec<(i32, i32)>>()
                    .first()
                    .unwrap();
                guard.x = current_position.1 as usize;
                guard.y = current_position.0 as usize;
                guard.orientation = match area_map[guard.y].get(guard.x).unwrap() {
                    '^' => Orientation::Up,
                    'v' => Orientation::Down,
                    '>' => Orientation::Right,
                    '<' => Orientation::Left,
                    _ => exit(1),
                };
                // println!("guard: {:?}", guard);
                loop {
                    // println!("round!");
                    // println!("guard: {:?}", guard);

                    let (dx, dy): (i32, i32) = match guard.orientation {
                        Orientation::Up => (0, -1),
                        Orientation::Down => (0, 1),
                        Orientation::Right => (1, 0),
                        Orientation::Left => (-1, 0),
                    };
                    let update = update_position(guard.x, guard.y, dx, dy, max_x, max_y);
                    if update.is_none() {
                        area_map[guard.y][guard.x] = 'X';
                        // counter += 1;
                        // println!("--");
                        // print_map(&area_map);
                        break;
                    } else {
                        let (nx, ny) = update.unwrap();
                        if *area_map[ny].get(nx).unwrap() == '#'
                            || *area_map[ny].get(nx).unwrap() == 'O'
                        {
                            guard.orientation = match guard.orientation {
                                Orientation::Up => Orientation::Right,
                                Orientation::Down => Orientation::Left,
                                Orientation::Right => Orientation::Down,
                                Orientation::Left => Orientation::Up,
                            };
                        } else {
                            pass_count[guard.y][guard.x] += 1;
                            if pass_count[guard.y][guard.x] > 30 {
                                counter += 1;
                                break;
                            }
                            area_map[guard.y][guard.x] = 'X';
                            guard.x = nx;
                            guard.y = ny;
                        }
                        match guard.orientation {
                            Orientation::Up => area_map[guard.y][guard.x] = '^',
                            Orientation::Down => area_map[guard.y][guard.x] = 'v',
                            Orientation::Right => area_map[guard.y][guard.x] = '>',
                            Orientation::Left => area_map[guard.y][guard.x] = '<',
                        };
                    }
                    // println!("--");
                    // print_map(&area_map);
                }
            }
        }
        println!("Solution part 2: {}", counter);
    }
}
