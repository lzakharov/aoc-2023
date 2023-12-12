use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day12.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut result = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            let (record, groups) = line.split_at(line.find(' ').unwrap());
            let groups: Vec<i32> = groups
                .trim()
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect();

            result += solve(record, &groups)
        }
    }

    println!("{result}");
}

fn solve(record: &str, groups: &[i32]) -> i32 {
    if groups.len() == 0 {
        return !record.contains('#') as i32;
    }
    if record.len() == 0 {
        return 0;
    }

    let mut group = groups[0];
    let chars: Vec<char> = record.chars().collect();

    let mut j = 0;
    while j < record.len() && chars[j] == '.' {
        j += 1;
    }
    if j == record.len() {
        return 0;
    }

    let mut result = 0;

    if chars[j] == '?' && j + 1 < record.len() {
        result += solve(&record[j + 1..], groups);
    }

    while j < record.len() && group > 0 && chars[j] != '.' {
        group -= 1;
        j += 1;
    }

    if group == 0 {
        if j == record.len() {
            result += (groups.len() == 1) as i32;
        } else if chars[j] != '#' {
            result += solve(&record[j + 1..], &groups[1..]);
        }
    }

    result
}
