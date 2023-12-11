use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day05.txt").unwrap();
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    let mut nums: Vec<i64> = (&lines.next().unwrap().unwrap()[7..])
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut used = vec![false; nums.len()];

    for line in lines {
        if let Ok(line) = line {
            if line.is_empty() {
                used.fill(false);
                continue;
            }

            if line.starts_with(|x: char| !x.is_digit(10)) {
                continue;
            }

            let row: Vec<i64> = line
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect();

            for i in 0..nums.len() {
                let num = nums[i];
                if !used[i] && num >= row[1] && num < row[1] + row[2] {
                    nums[i] = row[0] + (num - row[1]);
                    used[i] = true;
                }
            }
        }
    }

    println!("{}", nums.iter().min().unwrap());
}
