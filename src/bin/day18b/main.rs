use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day18.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut perimeter = 0;
    let mut vertices = vec![(0, 0)];

    for line in reader.lines() {
        if let Ok(line) = line {
            let d = i64::from_str_radix(&line[line.len() - 7..line.len() - 2], 16).unwrap();
            let direction = line.as_bytes()[line.len() - 2];

            perimeter += d;
            let v = vertices[vertices.len() - 1];
            vertices.push(match direction {
                b'3' => (v.0 - d, v.1),
                b'1' => (v.0 + d, v.1),
                b'2' => (v.0, v.1 - d),
                b'0' => (v.0, v.1 + d),
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
