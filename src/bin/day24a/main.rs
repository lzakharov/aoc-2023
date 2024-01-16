use std::io::BufRead;
use std::{fs::File, io};

const MIN: f64 = 200000000000000.;
const MAX: f64 = 400000000000000.;

fn main() {
    let file = File::open("input/day24.txt").unwrap();
    let reader = io::BufReader::new(file);

    let vectors: Vec<Vector> = reader
        .lines()
        .map(|line| {
            let parsed: Vec<f64> = line
                .unwrap()
                .split(&[',', '@'])
                .map(|x| x.trim().parse().unwrap())
                .collect();
            Vector::new(parsed[0], parsed[1], parsed[3], parsed[4])
        })
        .collect();

    let mut result = 0;

    for (i, a) in vectors.iter().enumerate() {
        for b in vectors.iter().skip(i + 1) {
            if let Some(point) = a.intersection(b) {
                if point.x >= MIN && point.x <= MAX && point.y >= MIN && point.y <= MAX {
                    result += 1;
                }
            }
        }
    }

    println!("{result}");
}

#[derive(Debug)]
struct Vector {
    start: Point,
    vx: f64,
    vy: f64,
    line: Line,
}

impl Vector {
    fn new(x: f64, y: f64, vx: f64, vy: f64) -> Self {
        let start = Point { x, y };
        let a = vy;
        let b = -vx;
        let c = vx * y - vy * x;
        let line = Line { a, b, c };
        Vector {
            start,
            vx,
            vy,
            line,
        }
    }

    fn intersection(&self, other: &Vector) -> Option<Point> {
        if let Some(point) = self.line.intersection(&other.line) {
            if !self.contains(&point) || !other.contains(&point) {
                return None;
            }

            return Some(point);
        }

        None
    }

    fn contains(&self, point: &Point) -> bool {
        (point.x - self.start.x) / self.vx > 0. || (point.y - self.start.y) / self.vy > 0.
    }
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Line {
    a: f64,
    b: f64,
    c: f64,
}

impl Line {
    fn parallel(&self, other: &Line) -> bool {
        self.a * other.b == other.a * self.b
    }

    fn intersection(&self, other: &Line) -> Option<Point> {
        if self.parallel(other) {
            return None;
        }

        let m = self.a * other.b - other.a * self.b;
        let x = (other.c * self.b - self.c * other.b) / m;
        let y = (self.c * other.a - other.c * self.a) / m;

        Some(Point { x, y })
    }
}
