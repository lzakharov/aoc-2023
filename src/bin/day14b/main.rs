use std::collections::HashMap;
use std::io::BufRead;
use std::{fs::File, io};

const N: usize = 1_000_000_000;

fn main() {
    let file = File::open("input/day14.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut map: Map = reader
        .lines()
        .map(|s| s.unwrap().chars().collect())
        .collect();

    let mut history: Vec<Map> = vec![];
    let mut mem: HashMap<String, usize> = HashMap::new();

    for i in 0..N {
        cycle(&mut map);
        history.push(map.clone());
        let k = map.concat().iter().collect();
        if let Some(j) = mem.get(&k) {
            let result = total_load(&history[j + ((N - 1 - j) % (i - j))]);
            println!("{}", result);
            break;
        }
        mem.insert(k, i);
    }
}

type Map = Vec<Vec<char>>;

fn cycle(map: &mut Map) {
    let n = map.len() as i32;
    let m = map[0].len() as i32;

    tilt(map, Range::new(0, n), Range::new(0, m), 1, false);
    tilt(map, Range::new(0, m), Range::new(0, n), 1, true);
    tilt(map, Range::new(n - 1, -1), Range::new(0, m), -1, false);
    tilt(map, Range::new(m - 1, -1), Range::new(0, n), -1, true);
}

fn tilt(map: &mut Map, irange: Range, jrange: Range, step: i32, reverse: bool) {
    let mut h = vec![irange.from; jrange.to as usize];

    let mut i = irange.from;
    while i != irange.to {
        let mut j = jrange.from;
        while j != jrange.to {
            let (x, y) = if !reverse { (i, j) } else { (j, i) };
            let (x, y) = (x as usize, y as usize);
            match map[x][y] {
                'O' => {
                    map[x][y] = '.';
                    if !reverse {
                        map[h[y] as usize][y] = 'O';
                        h[y] += step;
                    } else {
                        map[x][h[x] as usize] = 'O';
                        h[x] += step;
                    }
                }
                '#' => h[j as usize] = i + step,
                _ => {} // .
            }
            j += jrange.step;
        }
        i += irange.step;
    }
}

fn total_load(map: &Map) -> usize {
    map.iter()
        .enumerate()
        .map(|(i, line)| (map.len() - i) * line.iter().filter(|x| **x == 'O').count())
        .sum()
}

struct Range {
    from: i32,
    to: i32,
    step: i32,
}

impl Range {
    fn new(from: i32, to: i32) -> Self {
        let step = if from < to { 1 } else { -1 };
        Range {
            from: from,
            to: to,
            step,
        }
    }
}
