use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day13.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut result = 0;
    let mut map: Vec<Vec<char>> = vec![];

    for line in reader.lines() {
        if let Ok(line) = line {
            if !line.is_empty() {
                map.push(line.chars().collect());
            } else {
                result += solve(&map);
                map.clear();
            }
        }
    }

    result += solve(&map);

    println!("{result}");
}

fn solve(map: &Vec<Vec<char>>) -> i32 {
    for j in 1..map[0].len() {
        let mut count = 0;

        for k in 0..j.min(map[0].len() - j) {
            if count > 1 {
                break;
            }

            for i in 0..map.len() {
                if map[i][j - k - 1] != map[i][j + k] {
                    count += 1;
                    if count > 1 {
                        break;
                    }
                }
            }
        }

        if count == 1 {
            return j as i32;
        }
    }

    for i in 1..map.len() {
        let mut count = 0;

        for k in 0..i.min(map.len() - i) {
            if count > 1 {
                break;
            }

            for j in 0..map[0].len() {
                if map[i - k - 1][j] != map[i + k][j] {
                    count += 1;
                    if count > 1 {
                        break;
                    }
                }
            }
        }

        if count == 1 {
            return 100 * i as i32;
        }
    }

    0
}
