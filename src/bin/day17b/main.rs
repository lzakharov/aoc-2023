use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::BufRead;
use std::{fs::File, io};

const UP: usize = 0;
const DOWN: usize = 1;
const LEFT: usize = 2;
const RIGHT: usize = 3;

fn main() {
    let file = File::open("input/day17.txt").unwrap();
    let reader = io::BufReader::new(file);

    let map: Map = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .as_bytes()
                .iter()
                .map(|c| (c - b'0') as usize)
                .collect()
        })
        .collect();

    let result = solve(&map);
    println!("{result}");
}

type Map = Vec<Vec<usize>>;

fn solve(map: &Map) -> usize {
    let n: usize = map.len();
    let m: usize = map[0].len();

    let mut heap = BinaryHeap::from([
        (Reverse(map[0][1]), (0, 1), RIGHT, 0),
        (Reverse(map[1][0]), (1, 0), DOWN, 0),
    ]);

    let mut visited = vec![vec![[[false; 10]; 4]; m]; n];

    while let Some((Reverse(loss), (i, j), direction, steps)) = heap.pop() {
        if i == n - 1 && j == m - 1 && steps >= 3 {
            return loss;
        }

        if visited[i][j][direction][steps] {
            continue;
        }
        visited[i][j][direction][steps] = true;

        if i > 0 && direction != DOWN {
            if direction == UP && steps < 9 {
                heap.push((Reverse(loss + map[i - 1][j]), (i - 1, j), UP, steps + 1));
            }
            if direction != UP && steps >= 3 {
                heap.push((Reverse(loss + map[i - 1][j]), (i - 1, j), UP, 0));
            }
        }
        if i < n - 1 && direction != UP {
            if direction == DOWN && steps < 9 {
                heap.push((Reverse(loss + map[i + 1][j]), (i + 1, j), DOWN, steps + 1));
            }
            if direction != DOWN && steps >= 3 {
                heap.push((Reverse(loss + map[i + 1][j]), (i + 1, j), DOWN, 0));
            }
        }
        if j > 0 && direction != RIGHT {
            if direction == LEFT && steps < 9 {
                heap.push((Reverse(loss + map[i][j - 1]), (i, j - 1), LEFT, steps + 1));
            }
            if direction != LEFT && steps >= 3 {
                heap.push((Reverse(loss + map[i][j - 1]), (i, j - 1), LEFT, 0));
            }
        }
        if j < m - 1 && direction != LEFT {
            if direction == RIGHT && steps < 9 {
                heap.push((Reverse(loss + map[i][j + 1]), (i, j + 1), RIGHT, steps + 1));
            }
            if direction != RIGHT && steps >= 3 {
                heap.push((Reverse(loss + map[i][j + 1]), (i, j + 1), RIGHT, 0));
            }
        }
    }

    unreachable!()
}
