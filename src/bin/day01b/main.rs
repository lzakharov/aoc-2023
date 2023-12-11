use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day01.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut result = 0;

    let re = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let re2 = Regex::new(r"enin|thgie|neves|xis|evif|ruof|eerht|owt|eno|\d").unwrap();
    let m = HashMap::from([
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
        ("eno", 1),
        ("owt", 2),
        ("eerht", 3),
        ("ruof", 4),
        ("evif", 5),
        ("xis", 6),
        ("neves", 7),
        ("thgie", 8),
        ("enin", 9),
    ]);

    for line in reader.lines() {
        if let Ok(line) = line {
            let a = m[re.find(&line).unwrap().as_str()];
            let rev = &line.chars().rev().collect::<String>();
            let b = m[re2.find(&rev).unwrap().as_str()];
            result += 10 * a + b;
        }
    }

    println!("{result}");
}
