use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day16.txt").unwrap();
    let reader = io::BufReader::new(file);

    let map: Map = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let n = map.len();
    let m = map[0].len();

    let mut used = vec![vec![[false; 4]; m]; n];

    let mut result = 0;

    for i in 0..n {
        result = result.max(solve(&map, (i as isize, 0), Direction::Right, &mut used));
        result = result.max(solve(
            &map,
            (i as isize, (m - 1) as isize),
            Direction::Right,
            &mut used,
        ));
    }

    for j in 0..m {
        result = result.max(solve(&map, (0, j as isize), Direction::Down, &mut used));
        result = result.max(solve(
            &map,
            ((n - 1) as isize, j as isize),
            Direction::Up,
            &mut used,
        ));
    }

    println!("{result}");
}

type Map = Vec<Vec<char>>;

type Used = Vec<Vec<[bool; 4]>>;

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

fn solve(map: &Map, position: (isize, isize), direction: Direction, used: &mut Used) -> usize {
    walk(&map, position, direction, used);
    let result = energy(&used);
    clear(used);
    result
}

fn walk(map: &Map, (i, j): (isize, isize), direction: Direction, used: &mut Used) {
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

fn energy(used: &Used) -> usize {
    used.iter()
        .map(|line| line.iter().filter(|x| x.contains(&true)).count())
        .sum()
}

fn clear(used: &mut Used) {
    used.iter_mut()
        .for_each(|line| line.iter_mut().for_each(|x| x.fill(false)));
}
