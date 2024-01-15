use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day23.txt").unwrap();
    let reader = io::BufReader::new(file);

    let map: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let (n, m) = (map.len(), map[0].len());
    let mut edges: HashMap<Point, HashSet<Point>> = HashMap::new();
    let mut distances: HashMap<(Point, Point), isize> = HashMap::new();

    for x in 0..n {
        for y in 0..m {
            if map[x][y] == '#' {
                continue;
            }

            if x > 0 && map[x - 1][y] != '#' {
                add_edge(&mut edges, &mut distances, Point(x, y), Point(x - 1, y), 1);
            }
            if x < map.len() - 1 && map[x + 1][y] != '#' {
                add_edge(&mut edges, &mut distances, Point(x, y), Point(x + 1, y), 1);
            }
            if y > 0 && map[x][y - 1] != '#' {
                add_edge(&mut edges, &mut distances, Point(x, y), Point(x, y - 1), 1);
            }
            if y < map[0].len() - 1 && map[x][y + 1] != '#' {
                add_edge(&mut edges, &mut distances, Point(x, y), Point(x, y + 1), 1);
            }
        }
    }

    loop {
        let points: Vec<Point> = edges
            .iter()
            .filter(|(_, neighbors)| neighbors.len() == 2)
            .map(|(point, _)| point)
            .copied()
            .collect();

        if points.is_empty() {
            break;
        }

        for point in points {
            let mut neighbors = edges[&point].iter();
            let a = neighbors.next().unwrap().clone();
            let b = neighbors.next().unwrap().clone();
            let d = distances[&(a, point)] + distances[&(point, b)];
            edges.remove(&point);
            edges.get_mut(&a).map(|points| points.remove(&point));
            edges.get_mut(&b).map(|points| points.remove(&point));
            distances.remove(&(a, point));
            distances.remove(&(point, a));
            add_edge(&mut edges, &mut distances, a, b, d);
        }
    }

    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let result = walk((n, m), &edges, &distances, &mut visited, Point(0, 1));
    println!("{}", result);
}

fn add_edge(
    edges: &mut HashMap<Point, HashSet<Point>>,
    distances: &mut HashMap<(Point, Point), isize>,
    a: Point,
    b: Point,
    distance: isize,
) {
    edges.entry(a).or_insert(HashSet::new()).insert(b);
    distances.insert((a, b), distance);
    edges.entry(b).or_insert(HashSet::new()).insert(a);
    distances.insert((b, a), distance);
}

fn walk(
    (n, m): (usize, usize),
    edges: &HashMap<Point, HashSet<Point>>,
    distances: &HashMap<(Point, Point), isize>,
    visited: &mut Vec<Vec<bool>>,
    current: Point,
) -> isize {
    let Point(x, y) = current;

    if x == n - 1 && y == m - 2 {
        return 0;
    }

    visited[x][y] = true;

    if !edges.contains_key(&current) {
        return isize::MIN;
    }

    let result = edges[&current]
        .iter()
        .map(|next| {
            if visited[next.0][next.1] {
                return isize::MIN;
            }
            let d = distances[&(current, *next)];
            d + walk((n, m), edges, distances, visited, *next)
        })
        .max()
        .unwrap_or(isize::MIN);

    visited[x][y] = false;

    result
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(usize, usize);
