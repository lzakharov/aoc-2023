use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day02.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut result = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            let (game, sets) = line.split_at(line.find(':').unwrap());
            let id = game[5..].parse::<i32>().unwrap();

            let mut ok = true;

            for set in sets[2..].split(';') {
                let mut red = 0;
                let mut blue = 0;
                let mut green = 0;

                for subset in set.trim().split(", ") {
                    let (n, color) = subset.split_at(subset.find(' ').unwrap());
                    let n = n.parse::<i32>().unwrap();
                    match &color[1..] {
                        "red" => red += n,
                        "blue" => blue += n,
                        "green" => green += n,
                        _ => unreachable!(),
                    }
                }

                if !(red <= 12 && green <= 13 && blue <= 14) {
                    ok = false;
                }
            }

            if ok {
                result += id;
            }
        }
    }

    println!("{result}");
}
