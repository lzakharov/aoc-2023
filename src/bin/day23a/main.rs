use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day23.txt").unwrap();
    let reader = io::BufReader::new(file);

    let map: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    println!("{}", walk(&map, &mut visited, (0, 1)));
}

fn walk(map: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, (x, y): (usize, usize)) -> isize {
    if x == map.len() - 1 && y == map[0].len() - 2 {
        return 0;
    }

    visited[x][y] = true;

    let result = 1 + match map[x][y] {
        '^' if !visited[x - 1][y] => walk(map, visited, (x - 1, y)),
        '>' if !visited[x][y + 1] => walk(map, visited, (x, y + 1)),
        'v' if !visited[x + 1][y] => walk(map, visited, (x + 1, y)),
        '<' if !visited[x][y - 1] => walk(map, visited, (x, y - 1)),
        '.' => {
            let mut neighbors = Vec::with_capacity(4);
            if x > 0 && map[x - 1][y] != '#' && !visited[x - 1][y] {
                neighbors.push((x - 1, y))
            }
            if x < map.len() - 1 && map[x + 1][y] != '#' && !visited[x + 1][y] {
                neighbors.push((x + 1, y))
            }
            if y > 0 && map[x][y - 1] != '#' && !visited[x][y - 1] {
                neighbors.push((x, y - 1))
            }
            if y < map[0].len() - 1 && map[x][y + 1] != '#' && !visited[x][y + 1] {
                neighbors.push((x, y + 1))
            }

            neighbors
                .into_iter()
                .map(|p| walk(map, visited, p))
                .max()
                .unwrap_or(isize::MIN)
        }
        _ => isize::MIN,
    };

    visited[x][y] = false;

    result
}
