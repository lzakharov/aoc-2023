use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day14.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut counts: Vec<usize> = vec![];
    let mut h: Vec<usize> = vec![];

    for line in reader.lines() {
        if let Ok(line) = line {
            if counts.is_empty() {
                h.resize(line.len(), 0);
            }
            counts.push(0);
            for (j, x) in line.chars().enumerate() {
                match x {
                    'O' => {
                        counts[h[j]] += 1;
                        h[j] += 1;
                    }
                    '#' => h[j] = counts.len(),
                    _ => {} // '.'
                }
            }
        }
    }

    let result: usize = counts
        .iter()
        .enumerate()
        .map(|(i, x)| (counts.len() - i) * x)
        .sum();

    println!("{result}");
}
