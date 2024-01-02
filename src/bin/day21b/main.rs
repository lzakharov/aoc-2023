// https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21

use std::collections::VecDeque;
use std::io::BufRead;
use std::{fs::File, io};

const STEPS: usize = 26501365;

fn main() {
    let file = File::open("input/day21.txt").unwrap();
    let reader = io::BufReader::new(file);

    let garden: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let n = garden.len();
    let mut visited = vec![vec![false; n]; n];
    let start = find_start(&garden);

    let mut q = VecDeque::new();
    q.push_back((start, 0));

    let mut even = 0;
    let mut even_corners = 0;
    let mut odd = 0;
    let mut odd_corners = 0;

    while let Some(((i, j), steps)) = q.pop_front() {
        if steps > STEPS || visited[i][j] || garden[i][j] == '#' {
            continue;
        }

        visited[i][j] = true;

        if steps % 2 == 0 {
            even += 1;
            if steps > 65 {
                even_corners += 1;
            }
        } else {
            odd += 1;
            if steps > 65 {
                odd_corners += 1;
            }
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
        if j < n - 1 {
            q.push_back(((i, j + 1), steps + 1));
        }
    }

    let k = (STEPS - (n / 2)) / n;

    let result =
        ((k + 1) * (k + 1)) * odd + (k * k) * even - (k + 1) * odd_corners + k * even_corners;

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
