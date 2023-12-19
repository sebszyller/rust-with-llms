use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn day_two() -> Result<(), Error> {
    println!("day two innit");
    let from_file = read_text().unwrap();

    let games: Vec<Game> = from_file.iter().map(game_to_struct).collect();
    // let zipped: Vec<(&String, &Game)> = from_file.iter().zip(games.iter()).collect();
    let mut id_accum = 0;
    let mut total_power = 0;
    // for (gf, g) in zipped {
        // println!("Game {} | R: {} G: {} B: {}", g.id, g.reds.iter().sum::<u32>(), g.greens.iter().sum::<u32>(), g.blues.iter().sum::<u32>());
    for g in games.iter() {
        let power_mult = g.reds.iter().max().unwrap() * g.greens.iter().max().unwrap() * g.blues.iter().max().unwrap();
        total_power += power_mult;
        if g.valid {
            id_accum += g.id;
        }
    }
    println!("Valid IDs sum: {}", id_accum);
    println!("Total power: {}", total_power);
    Ok(())
}

fn read_text() -> Result<Vec<String>, Error> {
    let path = "data/aoc/day2.txt";
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let lines = reader.lines().filter_map(|line| line.ok()).collect();
    Ok(lines)
}

struct Game {
    id: u32,
    reds: Vec<u32>,
    greens: Vec<u32>,
    blues: Vec<u32>,
    valid: bool
}

fn game_to_struct(str: &String) -> Game {
    let re = Regex::new(r" (?<id>\d+):").unwrap();
    let caps = re.captures(str).unwrap();
    let id_str = &caps["id"];

    let mut reds: Vec<u32> = Vec::new();
    let mut greens: Vec<u32> = Vec::new();
    let mut blues: Vec<u32> = Vec::new();

    let mut valid_game: bool = true;
    for draw in str.split(';') {
        let (red, green, blue) = count_balls(draw);
        reds.push(red);
        greens.push(green);
        blues.push(blue);
        if (red > 12) || (green > 13) || (blue > 14) {
            valid_game = false;
        }
    }
    Game {
        id: parse_to_u32(id_str),
        reds: reds,
        greens: greens,
        blues: blues,
        valid: valid_game
    }
}

fn parse_to_u32(s: &str) -> u32 {
    s.parse::<u32>().unwrap()
}

fn count_balls(draw: &str) -> (u32, u32, u32) {
    (
        extract_colour_with_re(draw, "red"),
        extract_colour_with_re(draw, "green"),
        extract_colour_with_re(draw, "blue"),
    )
}

fn extract_colour_with_re(str: &str, colour: &str) -> u32 {
    let re_str = format!(r"(?<count>\d+) {}", colour);
    let re = Regex::new(&re_str).unwrap();
    let maybe_caps = re.captures(str);
    let count: u32 = match maybe_caps {
        Some(caps) => parse_to_u32(&caps["count"]),
        None => 0 as u32,
    };
    count
}

fn minimum_possible(g: &Game) -> u32 {
    1
}
