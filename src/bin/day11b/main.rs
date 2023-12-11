use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day11.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut map: Vec<Vec<char>> = vec![];
    let mut points: Vec<(usize, usize)> = vec![];
    let mut rows: Vec<usize> = vec![];
    let mut columns: Vec<usize> = vec![];

    for line in reader.lines() {
        if let Ok(line) = line {
            map.push(line.chars().collect());

            let i = map.len() - 1;
            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    points.push((i, j));
                }
            }

            let d = line.chars().all(|x| x == '.') as usize;
            if let Some(v) = rows.last() {
                rows.push(v.clone() + d);
            } else {
                rows.push(d);
            }
        }
    }

    for j in 0..map[0].len() {
        let d = (0..map.len()).map(|i| map[i][j]).all(|x| x == '.') as usize;
        if let Some(v) = columns.last() {
            columns.push(v.clone() + d);
        } else {
            columns.push(d);
        }
    }

    let mut result = 0;

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let a = points[i];
            let b = points[j];
            result += (a.0 as i64 - b.0 as i64).abs()
                + (a.1 as i64 - b.1 as i64).abs()
                + 999999 * (rows[a.0] as i64 - rows[b.0] as i64).abs()
                + 999999 * (columns[a.1] as i64 - columns[b.1] as i64).abs();
        }
    }

    println!("{}", result);
}
