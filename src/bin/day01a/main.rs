use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day01.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut result = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            let a = line
                .chars()
                .find(|x| x.is_digit(10))
                .unwrap()
                .to_digit(10)
                .unwrap();
            let b = line
                .chars()
                .rev()
                .find(|x| x.is_digit(10))
                .unwrap()
                .to_digit(10)
                .unwrap();
            result += 10 * a + b;
        }
    }

    println!("{result}");
}
