use std::collections::HashMap;
use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day08.txt").unwrap();
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    let instuctions: Vec<char> = lines.next().unwrap().unwrap().chars().collect();
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let mut nodes: Vec<String> = Vec::new();

    for line in lines.skip(1) {
        if let Ok(line) = line {
            let (node, left, right) = (&line[..3], &line[7..10], &line[12..15]);
            map.insert(node.to_string(), (left.to_string(), right.to_string()));
            if node.ends_with('A') {
                nodes.push(node.to_string());
            }
        }
    }

    let mut result = 1;

    for node in nodes {
        let mut steps = 0;

        let mut node = node;
        loop {
            for instruction in instuctions.iter() {
                let next = map.get(&node).unwrap();
                match instruction {
                    'L' => node = next.0.clone(),
                    'R' => node = next.1.clone(),
                    _ => unreachable!(),
                }
                steps += 1;
            }

            if node.ends_with('Z') {
                break;
            }
        }

        result = lcm(result, steps);
    }

    println!("{result}");
}

fn lcm(n: u64, m: u64) -> u64 {
    n * m / gcd(n, m)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}
