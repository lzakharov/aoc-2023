use std::collections::HashSet;
use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day04.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut d = vec![];

    for (i, line) in reader.lines().enumerate() {
        if let Ok(line) = line {
            let n = read(&line);
            if d.len() < i + n + 1 {
                d.resize(i + n + 1, 1);
            };
            for j in i + 1..i + n + 1 {
                d[j] += d[i];
            }
        }
    }

    println!("{}", d.iter().sum::<i32>());
}

fn read(s: &str) -> usize {
    let (left, right) = s.split_at(s.find('|').unwrap());
    let left: HashSet<i32> = left[left.find(':').unwrap() + 2..]
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let right: HashSet<i32> = right[2..]
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    left.intersection(&right).count()
}
