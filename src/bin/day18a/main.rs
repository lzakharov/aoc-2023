use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day18.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut perimeter = 0;
    let mut vertices = vec![(0, 0)];

    for line in reader.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split_whitespace().collect();
            assert_eq!(parts.len(), 3);
            let d: i64 = parts[1].parse().unwrap();

            perimeter += d;
            let v = vertices[vertices.len() - 1];
            vertices.push(match parts[0] {
                "U" => (v.0 - d, v.1),
                "D" => (v.0 + d, v.1),
                "L" => (v.0, v.1 - d),
                "R" => (v.0, v.1 + d),
                _ => unreachable!(),
            })
        }
    }

    // https://en.wikipedia.org/wiki/Pick's_theorem
    let result = area(&vertices) + perimeter / 2 + 1;
    println!("{result}");
}

// https://en.wikipedia.org/wiki/Shoelace_formula
fn area(vertices: &Vec<(i64, i64)>) -> i64 {
    let mut s = 0;
    for i in 0..vertices.len() - 1 {
        s += vertices[i].0 * vertices[i + 1].1 - vertices[i + 1].0 * vertices[i].1;
    }
    s -= vertices[0].0 * vertices[vertices.len() - 1].1;
    s.abs() / 2
}
