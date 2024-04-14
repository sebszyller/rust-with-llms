use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::usize;

pub fn day_three() -> Result<(), Error> {
    println!("day three innit");
    let from_file = read_text().unwrap();

    let mut engine: Vec<Vec<char>> = Vec::new();
    for line in from_file.iter() {
        engine.push(line.chars().collect());
    }

    let (sum, sum_stars) = traverse(&engine);
    println!("Part I: {}", sum);
    println!("Part II: {}", sum_stars);
    Ok(())
}

fn read_text() -> Result<Vec<String>, Error> {
    let path = "data/aoc/day3.txt";
    // let path = "data/aoc/day3small.txt";
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let lines = reader.lines().filter_map(|line| line.ok()).collect();
    Ok(lines)
}

fn traverse(engine: &Vec<Vec<char>>) -> (u64, u64) {
    let right_idx: usize = engine[0].len() - 1;
    let mut accum: u64 = 0;

    let mut stars_with_numbers: HashMap<(usize, usize), Vec<u64>> = HashMap::new();
    let mut accum_stars: u64 = 0;

    for (i, row) in engine.iter().enumerate() {
        let mut number_mode = false;
        let mut number_start: usize = 0;
        let mut number_end: usize;

        for (j, cell) in row.iter().enumerate() {
            if is_digit(cell) {
                if !number_mode {
                    number_mode = true;
                    number_start = j;
                }
            } else {
                if number_mode {
                    number_mode = false;
                    number_end = j - 1;

                    let slice: String = row[number_start..=number_end].iter().collect();
                    let parsed = parse_to_u64(&slice);
                    let (touching_p1, stars_loc) =
                        is_touching(i, number_start, number_end, &engine);
                    // part 1
                    if touching_p1 {
                        accum += parsed;
                    }
                    // part 2
                    if !stars_loc.is_empty() {
                        for tup in stars_loc {
                            stars_with_numbers.entry(tup).or_insert(vec![]).push(parsed);
                        }
                    }
                }
            }
            if (j == right_idx) && number_mode {
                number_end = j;
                let slice: String = row[number_start..=number_end].iter().collect();
                let parsed = parse_to_u64(&slice);
                let (touching_p1, stars_loc) = is_touching(i, number_start, number_end, &engine);
                // part 1
                if touching_p1 {
                    accum += parsed;
                }
                // part 2
                if !stars_loc.is_empty() {
                    for tup in stars_loc {
                        stars_with_numbers.entry(tup).or_insert(vec![]).push(parsed);
                    }
                }
            }
        }
    }
    // print_hashmap(stars_with_numbers);
    for (_, v) in stars_with_numbers.iter() {
        if v.len() == 2 {
            accum_stars += v[0] * v[1];
        }
    }

    (accum, accum_stars)
}

fn is_digit(c: &char) -> bool {
    let digits = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
    digits.contains(c)
}

fn is_touching(
    row: usize,
    number_start: usize,
    number_end: usize,
    engine: &Vec<Vec<char>>,
) -> (bool, HashSet<(usize, usize)>) {
    let top_idx: usize = 0;
    let left_idx: usize = 0;
    let right_idx: usize = engine[0].len() - 1;
    let bottom_idx: usize = engine.len() - 1;
    let scan_start: usize;
    let scan_end: usize;

    let mut touching = false;
    let mut stars: Vec<(usize, usize)> = Vec::new();

    // check left
    if number_start != left_idx {
        scan_start = number_start - 1;
        if !is_dot(&engine[row][scan_start]) && !is_digit(&engine[row][scan_start]) {
            touching = true;
        }
        if is_star(&engine[row][scan_start]) {
            stars.push((row, scan_start));
        }
    } else {
        scan_start = number_start;
    }

    // check right
    if number_end != right_idx {
        scan_end = number_end + 1;
        if !is_dot(&engine[row][scan_end]) && !is_digit(&engine[row][scan_end]) {
            touching = true;
        }
        if is_star(&engine[row][scan_end]) {
            stars.push((row, scan_end));
        }
    } else {
        scan_end = number_end;
    }

    // check above
    if row != top_idx {
        for i in scan_start..=scan_end {
            if !is_dot(&engine[row - 1][i]) {
                touching = true;
            }
            if is_star(&engine[row - 1][i]) {
                stars.push((row - 1, i));
            }
        }
    }

    // check below
    if row != bottom_idx {
        for i in scan_start..=scan_end {
            if !is_dot(&engine[row + 1][i]) {
                touching = true;
            }
            if is_star(&engine[row + 1][i]) {
                stars.push((row + 1, i));
            }
        }
    }
    let unique: HashSet<(usize, usize)> = stars.into_iter().collect();
    return (touching, unique);
}

fn is_dot(c: &char) -> bool {
    c == &'.'
}

fn is_star(c: &char) -> bool {
    c == &'*'
}

fn parse_to_u64(s: &str) -> u64 {
    s.parse::<u64>().unwrap()
}

fn print_hashmap(t: HashMap<(usize, usize), Vec<u64>>) {
    for (k, v) in t.iter() {
        let (i, j) = k;
        println!("");
        print!("K: ({}, {}) Vs: ", i, j);
        for i in v.iter() {
            print!("{} ", i);
        }
        println!("")
    }
}
