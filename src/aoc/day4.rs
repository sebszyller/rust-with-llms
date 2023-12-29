use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

#[derive(Clone)]
struct Card {
    nr: u32,
    winning: Vec<u32>,
    selected: Vec<u32>,
}

pub fn day_four() -> Result<(), Error> {
    println!("day four innit");
    let from_file = read_text().unwrap();

    let mut accum: u32 = 0;
    let mut cards: Vec<Card> = Vec::new();

    for line in from_file.iter() {
        let card = line_to_card(line, 10, 25); // 10-25 for the big file
                                               // let card = line_to_card(line, 5, 8); // 5-8 for the small file
        accum += count_points(matching_numbers(&card));
        cards.push(card);
    }
    let total_cards = count_cards(&cards, &cards);
    println!("Part I: {}", accum);
    println!("Part II: {}", total_cards);
    Ok(())
}

fn read_text() -> Result<Vec<String>, Error> {
    let path = "data/aoc/day4.txt";
    // let path = "data/aoc/day4small.txt";
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let lines = reader.lines().filter_map(|line| line.ok()).collect();
    Ok(lines)
}

fn line_to_card(str: &str, win_nr: u32, sel_nr: u32) -> Card {
    // yolo
    let mut re_str: String = String::new();
    re_str.push_str(r"Card\s+(?<number>\d+):");

    for i in 1..=win_nr {
        let formatted = format!(r"\s+(?<w{}>\d+)", i);
        re_str.push_str(&formatted);
    }
    re_str.push_str(r" \|");
    for i in 1..=sel_nr {
        let formatted = format!(r"\s+(?<s{}>\d+)", i);
        re_str.push_str(&formatted);
    }

    let re = Regex::new(&re_str).unwrap();
    let maybe_caps = re.captures(str);

    match maybe_caps {
        Some(caps) => Card {
            nr: parse_to_u32(&caps["number"]),
            winning: (2..=win_nr + 1)
                .map(|w| parse_to_u32(&caps[w as usize]))
                .collect(),
            selected: ((2 + win_nr)..=(1 + win_nr + sel_nr))
                .map(|s| parse_to_u32(&caps[(s) as usize]))
                .collect(),
        },
        None => panic!("How did this happen?!"),
    }
}

fn parse_to_u32(s: &str) -> u32 {
    s.parse::<u32>().unwrap()
}

fn matching_numbers(c: &Card) -> Vec<&u32> {
    let overlapping: Vec<&u32> = c
        .selected
        .iter()
        .filter(|&num| c.winning.contains(num))
        .collect();

    overlapping
}

fn count_points(matching_nums: Vec<&u32>) -> u32 {
    let base: u32 = 2;
    let l = matching_nums.len() as u32;
    if l == 0 {
        0
    } else {
        base.pow(l - 1)
    }
}

fn count_cards(held_cards: &Vec<Card>, all_cards: &Vec<Card>) -> u32 {
    let in_hand = held_cards.len() as u32;

    let mut follow_ups: u32 = 0;
    for c in held_cards.iter() {
        let mn: u32 = matching_numbers(c).len() as u32;
        if mn != 0 {
            let start: usize = c.nr as usize;
            let end: usize = (c.nr + mn) as usize;
            let card_subset: Vec<Card> = all_cards[start..end].to_vec(); // implicit clone() :(
            follow_ups += count_cards(&card_subset, all_cards)
        }
    }
    in_hand + follow_ups
}
