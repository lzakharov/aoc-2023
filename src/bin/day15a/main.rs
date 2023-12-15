use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day15.txt").unwrap();
    let reader = io::BufReader::new(file);

    let result: usize = reader.split(b',').map(|s| hash(&s.unwrap())).sum();
    println!("{result}");
}

fn hash(s: &[u8]) -> usize {
    s.iter().fold(0, |res, &x| (res + x as usize) * 17 % 256)
}
