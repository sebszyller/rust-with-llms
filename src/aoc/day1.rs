use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn day_one() -> Result<(), Error>{
    println!("day one innit");
    let from_file = read_text().unwrap();

    let mut accum_p1 = 0;
    for line in from_file.iter() {
        accum_p1 += process_line(line, true);
    }

    println!("Part I");
    println!("summed: {}", accum_p1);

    let mut accum_p2 = 0;
    for line in from_file.iter() {
        accum_p2 += process_line(line, false);
    }

    println!("Part II");
    println!("summed: {}", accum_p2);
    Ok(())
}

fn read_text() -> Result<Vec<String>, Error> {
    let path = "data/aoc/day1.txt";
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let lines = reader
        .lines()
        .filter_map(|line| line.ok())
        .collect();
    Ok(lines)
}

fn process_line(str: &String, simple: bool) -> u32{
    let first: u32;
    let last: u32;

    if simple {
        let mut digits: Vec<u32> = Vec::new();
        for c in str.chars() {
            if is_digit(&c) {
                digits.push(c.to_digit(10).unwrap());
            }
        }
        first = *digits.first().unwrap();
        last = *digits.last().unwrap();
    } else {
        let matches = find_digit_like(str);

        let with_smallest_idx = matches
            .iter()
            .min_by(|l, r| l.idx.cmp(&r.idx))
            .unwrap();

        let with_largest_idx = matches
            .iter()
            .max_by(|l, r| l.idx.cmp(&r.idx))
            .unwrap();
        first = map_to_int(&with_smallest_idx.s);
        last = map_to_int(&with_largest_idx.s);
    }

    let summed = first * 10 + last;
    summed
}

fn is_digit(c: &char) -> bool {
    let digits = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    digits.contains(c)
}

struct Occurrence {
    s: String,
    idx: usize
}

fn find_digit_like(str: &String) -> Vec<Occurrence> {
    let digit_like = [ "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut matches: Vec<Occurrence> = Vec::new();
    for d in digit_like {
        match str.find(d) {
            Some(idx) => matches.push(Occurrence { s: d.to_string(), idx: idx}),
            None => continue
        }
        match str.rfind(d) {
            Some(idx) => matches.push(Occurrence { s: d.to_string(), idx: idx}),
            None => continue
        }
    }
    matches
}

fn map_to_int(k: &str) -> u32 {
    let map: HashMap<&str, u32>= vec![
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ].into_iter().collect();

    *map.get(k).unwrap()
}
