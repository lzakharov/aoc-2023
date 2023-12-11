use std::collections::HashMap;
use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day08.txt").unwrap();
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    let instuctions: Vec<char> = lines.next().unwrap().unwrap().chars().collect();
    let mut map: HashMap<String, (String, String)> = HashMap::new();

    for line in lines.skip(1) {
        if let Ok(line) = line {
            let (node, left, right) = (&line[..3], &line[7..10], &line[12..15]);
            map.insert(node.to_string(), (left.to_string(), right.to_string()));
        }
    }

    let mut node = String::from("AAA");
    let mut result = 0;

    loop {
        for instruction in instuctions.iter() {
            let next = map.get(&node).unwrap();
            match instruction {
                'L' => node = next.0.clone(),
                'R' => node = next.1.clone(),
                _ => unreachable!(),
            }
            result += 1;
        }

        if node == "ZZZ" {
            break;
        }
    }

    println!("{result}");
}
