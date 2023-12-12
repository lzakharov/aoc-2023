use std::collections::HashMap;
use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day12.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut result = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            let (record, groups) = line.split_at(line.find(' ').unwrap());
            let mut record = record.to_string();
            record.push('?');
            let record = record.repeat(5);
            let groups = groups
                .trim()
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i64>>()
                .repeat(5);

            let mut cache: HashMap<(usize, usize), i64> = HashMap::new();
            result += solve(&mut cache, &record[..record.len() - 1], &groups, (0, 0))
        }
    }

    println!("{result}");
}

fn solve(
    cache: &mut HashMap<(usize, usize), i64>,
    record: &str,
    groups: &[i64],
    (i, j): (usize, usize),
) -> i64 {
    if j == groups.len() {
        return !record[i..].contains('#') as i64;
    }
    if i == record.len() {
        return 0;
    }

    let mut group = groups[j];
    let chars: Vec<char> = record[i..].chars().collect();

    let mut k = 0;
    while k < chars.len() && chars[k] == '.' {
        k += 1;
    }
    if k == chars.len() {
        return 0;
    }

    let mut result = 0;

    if chars[k] == '?' && k + 1 < chars.len() {
        let key = (i + k + 1, j);
        if let Some(n) = cache.get(&key) {
            result += n;
        } else {
            let n = solve(cache, record, groups, key);
            cache.insert(key, n);
            result += n;
        }
    }

    while k < chars.len() && group > 0 && chars[k] != '.' {
        group -= 1;
        k += 1;
    }

    if group == 0 {
        if k == chars.len() {
            result += (groups.len() == j + 1) as i64;
        } else if chars[k] != '#' {
            let key = (i + k + 1, j + 1);
            if let Some(n) = cache.get(&key) {
                result += n;
            } else {
                let n = solve(cache, record, groups, key);
                cache.insert(key, n);
                result += n;
            }
        }
    }

    result
}
