use std::collections::VecDeque;
use std::io::BufRead;
use std::{fs::File, io};

const STEPS: usize = 64;

fn main() {
    let file = File::open("input/day21.txt").unwrap();
    let reader = io::BufReader::new(file);

    let garden: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let (n, m) = (garden.len(), garden[0].len());
    let mut visited = vec![vec![false; m]; n];
    let start = find_start(&garden);

    let mut q = VecDeque::new();
    q.push_back((start, 0));

    let mut result = 0;

    while let Some(((i, j), steps)) = q.pop_front() {
        if steps > STEPS || visited[i][j] || garden[i][j] == '#' {
            continue;
        }

        visited[i][j] = true;

        if steps % 2 == 0 {
            result += 1;
        }

        if i > 0 {
            q.push_back(((i - 1, j), steps + 1));
        }
        if i < n - 1 {
            q.push_back(((i + 1, j), steps + 1));
        }
        if j > 0 {
            q.push_back(((i, j - 1), steps + 1));
        }
        if j < m - 1 {
            q.push_back(((i, j + 1), steps + 1));
        }
    }

    println!("{result}");
}

fn find_start(garden: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..garden.len() {
        for j in 0..garden[0].len() {
            if garden[i][j] == 'S' {
                return (i, j);
            }
        }
    }
    unreachable!()
}
