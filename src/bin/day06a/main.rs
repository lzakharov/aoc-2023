use std::io::BufRead;
use std::{fs::File, io};

const EPS: f64 = 1e-6;

fn main() {
    let file = File::open("input/day06.txt").unwrap();
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    let times = parse(&lines.next().unwrap().unwrap());
    let distances = parse(&lines.next().unwrap().unwrap());

    let mut result = 1;

    for i in 0..times.len() {
        let total = times[i] as f64;
        let distance = distances[i] as f64;
        let d = (total * total - (4f64 * distance)).sqrt();
        let min = ((total - d) / 2f64 + EPS).ceil();
        let max = ((total + d) / 2f64 - EPS).floor();
        result *= (max - min) as i64 + 1;
    }

    println!("{result}");
}

fn parse(line: &str) -> Vec<i64> {
    line[line.find(':').unwrap() + 1..]
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}
