mod day1;
mod day2;
mod day3;

use clap::{Arg, ArgMatches, Command};

use day1::day_one;
use day2::day_two;
use day3::day_three;

fn main() -> Result<(), std::io::Error> {
    let args: ArgMatches = cli().get_matches();
    let day: &str = &args.get_one::<String>("day").unwrap();

    match day {
        "1" => day_one(),
        "2" => day_two(),
        "3" => day_three(),
        _ => Ok(println!("That's a weird day: {}", day)),
    }
}

fn cli() -> Command {
    Command::new("Advent of Code 2023")
        .version("0.0.1")
        .author("Sebastian Szyller")
        .about("Learning Rust with LLMs by doing Advent of Code 2023")
        .arg(
            Arg::new("day")
                .required(true)
                .short('d')
                .long("day")
                .value_name("DAY_OF_MONTH")
                .help("Day of December corresponding to the AoC puzzle."),
        )
}
