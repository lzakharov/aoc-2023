use std::collections::VecDeque;
use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day10.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut map: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let n = map.len();
    let m = map[0].len();

    let mut cycle = vec![];

    let start = start(&map);
    let mut q = VecDeque::from([
        (Direction::North, start, vec![]),
        (Direction::South, start, vec![]),
        (Direction::East, start, vec![]),
        (Direction::West, start, vec![]),
    ]);

    while let Some((direction, position, mut path)) = q.pop_front() {
        match direction {
            Direction::North if position.0 > 0 => {
                let next = (position.0 - 1, position.1);
                path.push(next);
                match map[next.0][next.1] {
                    'S' if path.len() > cycle.len() => cycle = path,
                    '|' => q.push_back((Direction::North, next, path)),
                    '7' => q.push_back((Direction::West, next, path)),
                    'F' => q.push_back((Direction::East, next, path)),
                    _ => {}
                }
            }
            Direction::South if position.0 < n - 1 => {
                let next = (position.0 + 1, position.1);
                path.push(next);
                match map[next.0][next.1] {
                    'S' if path.len() > cycle.len() => cycle = path,
                    '|' => q.push_back((Direction::South, next, path)),
                    'J' => q.push_back((Direction::West, next, path)),
                    'L' => q.push_back((Direction::East, next, path)),
                    _ => {}
                }
            }
            Direction::West if position.1 > 0 => {
                let next = (position.0, position.1 - 1);
                path.push(next);
                match map[next.0][next.1] {
                    'S' if path.len() > cycle.len() => cycle = path,
                    '-' => q.push_back((Direction::West, next, path)),
                    'L' => q.push_back((Direction::North, next, path)),
                    'F' => q.push_back((Direction::South, next, path)),
                    _ => {}
                }
            }
            Direction::East if position.1 < m - 1 => {
                let next = (position.0, position.1 + 1);
                path.push(next);
                match map[next.0][next.1] {
                    'S' if path.len() > cycle.len() => cycle = path,
                    '-' => q.push_back((Direction::East, next, path)),
                    'J' => q.push_back((Direction::North, next, path)),
                    '7' => q.push_back((Direction::South, next, path)),
                    _ => {}
                }
            }
            _ => {}
        }
    }

    let next = cycle[0];
    let prev = cycle[cycle.len() - 2];

    if (start.0 > next.0 && start.1 < prev.1) || (start.0 > prev.0 && start.1 < next.1) {
        map[start.0][start.1] = 'L';
    }

    if (start.0 < next.0 && start.1 > prev.1) || (start.0 < prev.0 && start.1 > next.1) {
        map[start.0][start.1] = '7';
    }

    for point in cycle {
        match map[point.0][point.1] {
            'L' | '7' => map[point.0][point.1] = 'O',
            _ => map[point.0][point.1] = 'X',
        }
    }

    let mut result = 0;

    for i in 0..n {
        for j in 0..m {
            if map[i][j] == 'X' || map[i][j] == 'O' {
                continue;
            }

            let mut hits = 0;
            let mut x = i;
            let mut y = j;

            while x < n && y < m {
                if map[x][y] == 'X' {
                    hits += 1;
                }
                x += 1;
                y += 1;
            }

            result += hits % 2;
        }
    }

    println!("{result}");
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
