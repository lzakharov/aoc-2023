use std::io::BufRead;
use std::{fs::File, io};

const EPS: f64 = 1e-6;

fn main() {
    let file = File::open("input/day06.txt").unwrap();
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    let time = parse(&lines.next().unwrap().unwrap());
    let distance = parse(&lines.next().unwrap().unwrap());

    let d = (time * time - (4f64 * distance)).sqrt();
    let min = ((time - d) / 2f64 + EPS).ceil();
    let max = ((time + d) / 2f64 - EPS).floor();
    println!("{}", (max - min) as i64 + 1);
}

fn parse(line: &str) -> f64 {
    line.chars()
        .filter(|x| x.is_digit(10))
        .collect::<String>()
        .parse::<f64>()
        .unwrap()
}
