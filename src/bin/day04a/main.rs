use std::collections::HashSet;
use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day04.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut result = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            let (left, right) = line.split_at(line.find('|').unwrap());
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

            let n = left.intersection(&right).count();
            if n > 0 {
                result += 1 << (n - 1);
            }
        }
    }

    println!("{result}");
}
