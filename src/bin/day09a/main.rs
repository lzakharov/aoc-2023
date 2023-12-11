use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day09.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut result = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            result += solve(
                &line
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<i32>>(),
            );
        }
    }

    println!("{result}");
}

fn solve(seq: &[i32]) -> i32 {
    let mut d = vec![];
    for i in 0..seq.len() - 1 {
        d.push(seq[i + 1] - seq[i]);
    }

    if d.iter().all(|x| *x == 0) {
        return seq[seq.len() - 1];
    } else {
        return seq[seq.len() - 1] + solve(&d);
    }
}
