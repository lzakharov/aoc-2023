use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day13.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut result = 0;
    let mut m: Vec<Vec<char>> = vec![];

    for line in reader.lines() {
        if let Ok(line) = line {
            if !line.is_empty() {
                m.push(line.chars().collect());
            } else {
                result += solve(&m);
                m.clear();
            }
        }
    }

    result += solve(&m);

    println!("{result}");
}

fn solve(m: &Vec<Vec<char>>) -> i32 {
    for j in 1..m[0].len() {
        let mut ok = true;
        for k in 0..j.min(m[0].len() - j) {
            if !ok {
                break;
            }
            for i in 0..m.len() {
                if m[i][j - k - 1] != m[i][j + k] {
                    ok = false;
                    break;
                }
            }
        }
        if ok {
            return j as i32;
        }
    }

    for i in 1..m.len() {
        let mut ok = true;
        for k in 0..i.min(m.len() - i) {
            if m[i - k - 1] != m[i + k] {
                ok = false;
                break;
            }
        }
        if ok {
            return 100 * i as i32;
        }
    }

    0
}
