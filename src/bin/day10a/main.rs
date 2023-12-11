use std::collections::VecDeque;
use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day10.txt").unwrap();
    let reader = io::BufReader::new(file);

    let map: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let n = map.len();
    let m = map[0].len();

    let mut result = 0;

    let start = start(&map);
    let mut q = VecDeque::from([
        (Direction::North, start, 0),
        (Direction::South, start, 0),
        (Direction::East, start, 0),
        (Direction::West, start, 0),
    ]);

    while let Some((direction, position, d)) = q.pop_front() {
        match direction {
            Direction::North if position.0 > 0 => {
                let next = (position.0 - 1, position.1);
                match map[next.0][next.1] {
                    'S' => result = result.max(d),
                    '|' => q.push_back((Direction::North, next, d + 1)),
                    '7' => q.push_back((Direction::West, next, d + 1)),
                    'F' => q.push_back((Direction::East, next, d + 1)),
                    _ => {}
                }
            }
            Direction::South if position.0 < n - 1 => {
                let next = (position.0 + 1, position.1);
                match map[next.0][next.1] {
                    'S' => result = result.max(d),
                    '|' => q.push_back((Direction::South, next, d + 1)),
                    'J' => q.push_back((Direction::West, next, d + 1)),
                    'L' => q.push_back((Direction::East, next, d + 1)),
                    _ => {}
                }
            }
            Direction::West if position.1 > 0 => {
                let next = (position.0, position.1 - 1);
                match map[next.0][next.1] {
                    'S' => result = result.max(d),
                    '-' => q.push_back((Direction::West, next, d + 1)),
                    'L' => q.push_back((Direction::North, next, d + 1)),
                    'F' => q.push_back((Direction::South, next, d + 1)),
                    _ => {}
                }
            }
            Direction::East if position.1 < m - 1 => {
                let next = (position.0, position.1 + 1);
                match map[next.0][next.1] {
                    'S' => result = result.max(d),
                    '-' => q.push_back((Direction::East, next, d + 1)),
                    'J' => q.push_back((Direction::North, next, d + 1)),
                    '7' => q.push_back((Direction::South, next, d + 1)),
                    _ => {}
                }
            }
            _ => {}
        }
    }

    println!("{}", (result + 1) / 2);
}

fn start(map: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'S' {
                return (i, j);
            }
        }
    }
    unreachable!()
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}
