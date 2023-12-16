use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day16.txt").unwrap();
    let reader = io::BufReader::new(file);

    let map: Map = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let mut used = vec![vec![[false; 4]; map[0].len()]; map.len()];

    walk(&map, (0, 0), Direction::Right, &mut used);

    for line in used.iter() {
        for x in line.iter() {
            print!("{}", if x.contains(&true) { '#' } else { '.' });
        }
        println!();
    }

    let result: usize = used
        .iter()
        .map(|line| line.iter().filter(|x| x.contains(&true)).count())
        .sum();

    println!("{result}");
}

type Map = Vec<Vec<char>>;

#[derive(Debug, Clone)]
enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl Direction {
    fn next(&self, (i, j): (isize, isize)) -> (isize, isize) {
        match self {
            Direction::Up => (i - 1, j),
            Direction::Down => (i + 1, j),
            Direction::Left => (i, j - 1),
            Direction::Right => (i, j + 1),
        }
    }
}

fn walk(map: &Map, (i, j): (isize, isize), direction: Direction, used: &mut Vec<Vec<[bool; 4]>>) {
    if !check(map, (i, j)) || used[i as usize][j as usize][direction.clone() as usize] {
        return;
    }

    used[i as usize][j as usize][direction.clone() as usize] = true;
    let next = direction.next((i, j));

    match map[i as usize][j as usize] {
        '.' => walk(map, next, direction, used),
        '|' => match direction {
            Direction::Up | Direction::Down => walk(map, next, direction, used),
            Direction::Left | Direction::Right => {
                walk(map, (i - 1, j), Direction::Up, used);
                walk(map, (i + 1, j), Direction::Down, used);
            }
        },
        '-' => match direction {
            Direction::Left | Direction::Right => walk(map, next, direction, used),
            Direction::Up | Direction::Down => {
                walk(map, (i, j - 1), Direction::Left, used);
                walk(map, (i, j + 1), Direction::Right, used);
            }
        },
        '/' => match direction {
            Direction::Up => walk(map, (i, j + 1), Direction::Right, used),
            Direction::Down => walk(map, (i, j - 1), Direction::Left, used),
            Direction::Left => walk(map, (i + 1, j), Direction::Down, used),
            Direction::Right => walk(map, (i - 1, j), Direction::Up, used),
        },
        '\\' => match direction {
            Direction::Up => walk(map, (i, j - 1), Direction::Left, used),
            Direction::Down => walk(map, (i, j + 1), Direction::Right, used),
            Direction::Left => walk(map, (i - 1, j), Direction::Up, used),
            Direction::Right => walk(map, (i + 1, j), Direction::Down, used),
        },
        _ => unreachable!(),
    }
}

fn check(map: &Map, (i, j): (isize, isize)) -> bool {
    i >= 0 && ((i as usize) < map.len()) && j >= 0 && ((j as usize) < map[0].len())
}
