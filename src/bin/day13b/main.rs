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
    let n = map.len();
    let m = map[0].len();

    100 * count(map, n, m, false) + count(map, m, n, true)
}

fn count(map: &Vec<Vec<char>>, n: usize, m: usize, reverse: bool) -> i32 {
    for i in 1..n {
        let mut count = 0;

        for k in 0..i.min(n - i) {
            if count > 1 {
                break;
            }

            for j in 0..m {
                if (!reverse && map[i - k - 1][j] != map[i + k][j])
                    || (reverse && map[j][i - k - 1] != map[j][i + k])
                {
                    count += 1;
                    if count > 1 {
                        break;
                    }
                }
            }
        }

        if count == 1 {
            return i as i32;
        }
    }

    0
}
